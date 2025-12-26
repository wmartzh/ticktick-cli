use crate::{
    client, config,
    tick_tick_api::{Project, ProjectTaskResponse, Task},
};

pub async fn get_project_tasks(
    project_id: &String,
) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    println!("{:?}", project_id);
    let response = client::client()
        .get(format!(
            "{}/open/v1/project/{}/data",
            &config::get().api_host,
            project_id
        ))
        .send()
        .await?;

    let body = response.text().await?;

    let project_data: ProjectTaskResponse = serde_json::from_str(&body)?;

    if let Some(tasks) = project_data.tasks {
        return Ok(tasks);
    }
    Err("No tasks found on project".into())
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

fn is_white_list_value(name: &str) -> bool {
    let white_list: Vec<String> = vec![String::from("inbox")];

    let value = white_list.iter().find(|&v| v == name);

    value.is_some()
}

async fn get_project_id(project_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let projects = get_projects().await?;
    if let Some(current_project) = projects.iter().find(|p| {
        let lower_case_name = &p.name.to_lowercase();
        return lower_case_name.contains(&project_name.to_lowercase());
    }) {
        return Ok(current_project.id.clone());
    }
    Err("Cannot get project id".into())
}

pub async fn get_project(name: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let default_project = &config::AppConfig::load()?.default_project;

    if let Some(user_project) = name {
        if is_white_list_value(&user_project) {
            return Ok(user_project);
        }
        let project_id = get_project_id(&user_project).await?;
        return Ok(project_id);
    } else {
        let project_id = get_project_id(&default_project).await?;
        return Ok(project_id);
    }
}
