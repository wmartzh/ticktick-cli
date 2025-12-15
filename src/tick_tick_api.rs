use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub kind: Option<String>,
    pub sort_order: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskBody {
    pub title: String,
    pub project_id: Option<String>,
    pub tags: Vec<String>,
    pub due_date: Option<String>,
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub etag: String,
    pub is_all_day: bool,
    pub content: Option<String>,
    pub title: String,
    pub kind: String,
    pub due_date: Option<String>,
    pub priority: u32,
    pub project_id: String,
    pub status: u32,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub sort_order: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTaskResponse {
    pub columns: Vec<Column>,
    pub tasks: Vec<Task>,
}
