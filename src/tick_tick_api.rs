use serde::{Deserialize, Serialize};

use crate::{client, config};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
}

pub struct TickTickApi;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CreateTaskBody {
    title: String,
    project_id: Option<String>,
}

impl TickTickApi {
    pub async fn get_projects() -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let response = client::client()
            .get(format!("{}/open/v1/project", &config::get().api_host))
            .send()
            .await?;

        let projects: Vec<Project> = response.json().await?;
        println!("{:#?}", projects);
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
            let projects = self::TickTickApi::get_projects()
                .await
                .unwrap_or(Vec::new());
            if let Some(p) = projects.iter().find(|p| {
                let lower_name = p.name.to_lowercase();
                lower_name.contains(&name.to_lowercase())
            }) {
                project_id = Some(p.id.clone());
            }
        }
        Ok(project_id)
    }
    pub async fn create(
        title: String,
        project_id: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let body = CreateTaskBody { title, project_id };

        let _ = client::client()
            .post(format!("{}/open/v1/task", &config::get().api_host))
            .json(&body)
            .send()
            .await?;
        Ok(())
    }
}
