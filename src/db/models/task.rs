use crate::db::models::project::{AssocProject, Project};
use crate::db::models::user::{AssocUser, User};
use crate::schema::tasks;
use chrono::NaiveDateTime;

#[derive(PartialEq, Debug, Queryable, Associations, Clone, Identifiable)]
#[belongs_to(parent = "User")]
#[belongs_to(parent = "AssocUser", foreign_key = "user_id")]
#[belongs_to(parent = "Project")]
#[belongs_to(parent = "AssocProject", foreign_key = "project_id")]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
    pub task_name: String,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
    pub progress: i16,
    pub order_number: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct TaskNewForm<'a> {
    pub project_id: i32,
    pub user_id: i32,
    pub task_name: &'a str,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
    pub progress: i16,
    pub order_number: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "tasks"]
pub struct TaskUpdateForm<'a> {
    pub user_id: Option<i32>,
    pub task_name: Option<&'a str>,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>,
    pub progress: Option<i16>,
    pub order_number: Option<f64>,
    pub updated_at: NaiveDateTime,
}
