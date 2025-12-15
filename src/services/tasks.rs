use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    client, config, services, tick_tick_api::CreateTaskBody, ui::tables::render_tasks, CreateArgs,
};

fn parse_flexible_date(input: &str) -> Result<NaiveDateTime, String> {
    // Format: YYYY-MM-DD HH:MM:SS
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S") {
        return Ok(dt);
    }

    // Format: YYYY-MM-DDTHH:MM:SS
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S") {
        return Ok(dt);
    }

    // Format: YYYY-MM-DD
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        // If successful, convert it to DateTime at midnight (00:00:00)
        // .and_hms_opt(0, 0, 0) handles invalid times safely
        return Ok(date.and_hms_opt(0, 0, 0).unwrap());
    }

    // 3. If everything failed
    Err(format!(
        "Could not parse date: '{}'. Expected YYYY-MM-DD or YYYY-MM-DD HH:MM:SS",
        input
    ))
}

pub async fn create_task(args: &CreateArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project_id: Option<String> = services::projects::get_project_id(args.project.clone())
        .await
        .unwrap_or(None);

    let mut body = CreateTaskBody {
        title: args.title.clone(),
        project_id,
        tags: args.tags.clone(),
        due_date: None,
    };

    if !args.due.is_empty() {
        let new_date = parse_flexible_date(&args.due)?;
        let date_str = new_date.format("%Y-%m-%dT%H:%M:%SZ").to_string();
        body.due_date = Some(date_str);
    }
    println!("{:?}", body);

    let _ = client::client()
        .post(format!("{}/open/v1/task", &config::get().api_host))
        .json(&body)
        .send()
        .await?;

    Ok(())
}

pub async fn get_tasks(project: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let project_id: Option<String> = services::projects::get_project_id(project)
        .await
        .unwrap_or(None);
    let tasks = services::projects::get_project_tasks(project_id).await?;

    render_tasks(&tasks);

    Ok(())
}
