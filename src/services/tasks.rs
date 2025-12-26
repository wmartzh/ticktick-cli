use chrono::{NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

use crate::{
    client, config, services,
    tick_tick_api::{CreateTaskBody, TaskPriority},
    ui::views::render_tasks,
    CreateArgs,
};

fn parse_flexible_date(input: &str, timezone: &str) -> Option<String> {
    // Parse the timezone
    let tz: Tz = timezone.parse().ok()?;

    // Format: YYYY-MM-DD HH:MMam/pm (e.g., "2025-12-16 2:00pm")
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %I:%M%p") {
        // Convert from local timezone to UTC
        let local_dt = tz.from_local_datetime(&dt).single()?;
        let utc_dt = local_dt.with_timezone(&chrono_tz::UTC);
        return Some(utc_dt.format("%Y-%m-%dT%H:%M:%S+0000").to_string());
    }

    // Format: YYYY-MM-DD (e.g., "2025-12-25")
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        // Convert to datetime at midnight in local timezone, then to UTC
        let naive_dt = date.and_hms_opt(0, 0, 0)?;
        let local_dt = tz.from_local_datetime(&naive_dt).single()?;
        let utc_dt = local_dt.with_timezone(&chrono_tz::UTC);
        return Some(utc_dt.format("%Y-%m-%dT%H:%M:%S+0000").to_string());
    }

    None
}

fn parse_priority(priority: &TaskPriority) -> u32 {
    let value = match priority {
        TaskPriority::Low => 1,
        TaskPriority::Mid => 3,
        TaskPriority::High => 5,
    };
    value
}

pub async fn create_task(args: &CreateArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project_id = services::projects::get_project(Some(args.project.clone())).await?;

    let mut body = CreateTaskBody {
        title: args.title.clone(),
        project_id: Some(project_id),
        tags: args.tags.clone(),
        due_date: None,
        time_zone: config::get().time_zone.clone(),
        priority: None,
    };

    if let Some(due) = &args.due {
        body.due_date = parse_flexible_date(due, &config::get().time_zone);
    }

    if let Some(pr) = &args.priority {
        body.priority = Some(parse_priority(pr));
    }

    let _ = client::client()
        .post(format!("{}/open/v1/task", &config::get().api_host))
        .json(&body)
        .send()
        .await?;

    Ok(())
}

pub async fn get_tasks(project: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let project_id = services::projects::get_project(project).await?;
    println!("using project {:?}\n", project_id);
    let tasks = services::projects::get_project_tasks(&project_id).await?;

    render_tasks(tasks)?;

    Ok(())
}
