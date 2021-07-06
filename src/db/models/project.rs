use crate::schema::projects;
use chrono::NaiveDateTime;

#[derive(Eq, PartialEq, Debug, Queryable, Clone, Identifiable)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub color: String,
    pub is_archived: bool,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Eq, PartialEq, Debug, Queryable, Clone, Identifiable)]
#[table_name = "projects"]
pub struct AssocProject {
    pub id: i32,
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct ProjectNewForm<'a> {
    pub title: &'a str,
    pub color: &'a str,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "projects"]
pub struct ProjectUpdateForm<'a> {
    pub title: Option<&'a str>,
    pub color: Option<&'a str>,
    pub is_archived: Option<bool>,
    pub started_at: Option<NaiveDateTime>,
    pub ended_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}
