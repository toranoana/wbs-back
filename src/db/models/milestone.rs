use crate::db::models::project::{AssocProject, Project};
use crate::schema::milestones;
use chrono::NaiveDateTime;

#[derive(Eq, PartialEq, Debug, Queryable, Associations, Clone, Identifiable)]
#[belongs_to(parent = "Project")]
#[belongs_to(parent = "AssocProject", foreign_key = "project_id")]
pub struct Milestone {
    pub id: i32,
    pub project_id: i32,
    pub confirmed_at: NaiveDateTime,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "milestones"]
pub struct MilestoneNewForm<'a> {
    pub project_id: i32,
    pub confirmed_at: NaiveDateTime,
    pub description: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "milestones"]
pub struct MilestoneUpdateForm<'a> {
    pub confirmed_at: Option<NaiveDateTime>,
    pub description: Option<&'a str>,
    pub updated_at: NaiveDateTime,
}
