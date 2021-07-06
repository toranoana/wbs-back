use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub user_id: i32,
    pub task_name: String,
    pub started_at: String,
    pub ended_at: String,
    pub progress: Option<i16>,
    pub order_number: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskImportRequest {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskImportResponse {
    pub inserted_tasks: Vec<Task>,
    pub error_lines: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectPath {
    pub project_id: i32,
}
