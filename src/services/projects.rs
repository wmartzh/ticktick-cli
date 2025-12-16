use crate::{
    client, config,
    tick_tick_api::{Project, ProjectTaskResponse, Task},
};

pub async fn get_project_tasks(
    project_id: Option<String>,
) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let name = project_id.unwrap_or("inbox".to_string());
    let response = client::client()
        .get(format!(
            "{}/open/v1/project/{}/data",
            &config::get().api_host,
            name
        ))
        .send()
        .await?;

    // Get the raw response text to debug
    let response_text = response.text().await?;

    // Try to deserialize
    let project_response: ProjectTaskResponse = serde_json::from_str(&response_text)?;

    Ok(project_response.tasks)
}

pub async fn get_project_task(
    project_id: Option<String>,
    id: &str,
) -> Result<Task, Box<dyn std::error::Error>> {
    let name = project_id.unwrap_or("inbox".to_string());
    let response = client::client()
        .get(format!(
            "{}/open/v1/project/{}/tastk/{}",
            &config::get().api_host,
            name,
            id
        ))
        .send()
        .await?;

    let task_response = response.json::<Task>().await?;
    Ok(task_response)
}

pub async fn get_projects() -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let response = client::client()
        .get(format!("{}/open/v1/project", &config::get().api_host))
        .send()
        .await?;

    let projects: Vec<Project> = response.json().await?;
    Ok(projects)
}

pub async fn get_project_id_by_name(
    name: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut project_id: Option<String> = None;

    if name.to_lowercase() == "inbox" {
        println!("⚠️Project not found using inbox");
        project_id = None
    } else {
        let projects = get_projects().await.unwrap_or(Vec::new());
        if let Some(p) = projects.iter().find(|p| {
            let lower_name = p.name.to_lowercase();
            lower_name.contains(&name.to_lowercase())
        }) {
            project_id = Some(p.id.clone());
        }
    }
    Ok(project_id)
}

pub async fn get_project_id(
    project: Option<String>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let default_project = config::AppConfig::load()?
        .default_project
        .unwrap_or("inbox".to_string());

    let project_name = project.unwrap_or(default_project.clone());

    let project_id = get_project_id_by_name(&project_name).await?;

    Ok(project_id)
}
