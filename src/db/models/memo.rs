use crate::db::models::task::Task;
use crate::db::models::user::{AssocUser, User};
use crate::schema::memos;
use chrono::NaiveDateTime;

#[derive(PartialEq, Debug, Queryable, Associations, Clone, Identifiable)]
#[belongs_to(parent = "User")]
#[belongs_to(parent = "AssocUser", foreign_key = "user_id")]
#[belongs_to(parent = "Task")]
// #[belongs_to(parent = "AssocProject", foreign_key = "project_id")]
pub struct Memo {
    pub id: i32,
    pub task_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "memos"]
pub struct MemoNewForm<'a> {
    pub task_id: i32,
    pub user_id: i32,
    pub content: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "memos"]
pub struct MemoUpdateForm<'a> {
    pub content: &'a str,
}
