use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Eq, PartialEq, Debug, Queryable, Clone, Identifiable)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub is_disabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Eq, PartialEq, Debug, Queryable, Clone, Identifiable)]
#[table_name = "users"]
pub struct AssocUser {
    pub id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserNewForm<'a> {
    pub display_name: &'a str,
}

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserUpdateForm<'a> {
    pub display_name: Option<&'a str>,
    pub is_disabled: Option<bool>,
    pub updated_at: NaiveDateTime,
}
