use crate::{config, tick_tick_api::TickTickApi};

pub async fn get_project_id(
    project: Option<String>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let default_project = config::AppConfig::load()?
        .default_project
        .unwrap_or("inbox".to_string());

    let project_name = project.unwrap_or(default_project.clone());

    let project_id = TickTickApi::get_project_id_by_name(&project_name).await?;

    Ok(project_id)
}
