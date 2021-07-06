use crate::db::models::*;
use crate::graphql::schema::task::Task;
use crate::graphql::schema::Context;
use chrono::{Local, ParseError};
use juniper::ID;
use std::convert::TryFrom;

pub struct User {
    id: i32,
    display_name: String,
    is_disabled: bool,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A User create")]
pub struct NewUser {
    display_name: String,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A User returns struct")]
impl User {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn display_name(&self) -> String {
        self.display_name.clone()
    }

    fn is_disabled(&self) -> bool {
        self.is_disabled
    }

    async fn tasks(&self, context: &Context) -> Vec<Task> {
        let task = context.loaders.user_tasks_loader.load(self.id).await;
        task.into_iter().map(|t| t.into()).collect()
    }
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A User update")]
pub struct UpdateUser {
    display_name: Option<String>,
    is_disabled: Option<bool>,
}

// GraphQLのUserをdieselのUserに変換するFromトレイト実装
impl From<user::User> for User {
    fn from(user: user::User) -> Self {
        Self {
            id: user.id,
            display_name: user.display_name,
            is_disabled: user.is_disabled,
        }
    }
}

/// GraphQLの構造体NewUserをdieselの構造体UserNewFormに変換するFromトレイト実装
impl<'a> TryFrom<&'a NewUser> for user::UserNewForm<'a> {
    type Error = ParseError;
    fn try_from(new_user: &'a NewUser) -> Result<Self, Self::Error> {
        Ok(Self {
            display_name: &new_user.display_name,
        })
    }
}

/// GraphQLの構造体UpdateUserをdieselの構造体UserUpdateFormに変換するFromトレイト実装
impl<'a> From<&'a UpdateUser> for user::UserUpdateForm<'a> {
    fn from(update_user: &'a UpdateUser) -> Self {
        Self {
            display_name: update_user.display_name.as_ref().map(AsRef::as_ref),
            is_disabled: update_user.is_disabled,
            updated_at: Local::now().naive_local(),
        }
    }
}
