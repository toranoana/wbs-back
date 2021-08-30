use crate::schema::holidays;
use chrono::NaiveDateTime;

#[derive(PartialEq, Debug, Queryable, Associations, Clone, Identifiable)]
// #[belongs_to(parent = "AssocProject", foreign_key = "project_id")]
pub struct Holiday {
    pub id: i32,
    pub holiday_name: String,
    pub target_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "holidays"]
pub struct HolidayNewForm<'a> {
    pub holiday_name: &'a str,
    pub target_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name = "holidays"]
pub struct HolidayUpdateForm<'a> {
    pub holiday_name: Option<&'a str>,
    pub target_at: Option<NaiveDateTime>,
}
