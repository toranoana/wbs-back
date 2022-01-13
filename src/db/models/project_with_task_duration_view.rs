use crate::view_schema::project_with_task_duration_view;
use chrono::NaiveDateTime;
use serde::{Serialize};

#[derive(Eq, PartialEq, Debug, Queryable, Clone, Identifiable, Serialize)]
#[table_name = "project_with_task_duration_view"]
pub struct ProjectWithTaskDurationView {
    pub id: i32,
    pub title: String,
    pub color: String,
    pub is_archived: bool,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
    pub min_started_at: Option<NaiveDateTime>,
    pub max_ended_at: Option<NaiveDateTime>,
}
