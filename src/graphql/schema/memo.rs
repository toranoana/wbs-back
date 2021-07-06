use crate::db::models::*;
use crate::graphql::schema::task::Task;
use crate::graphql::schema::Context;
use chrono::{Local, ParseError};
use juniper::ID;
use std::convert::TryFrom;

pub struct Memo {
    id: i32,
    task_id: i32,
    user_id: i32,
    content: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Memo create")]
pub struct NewMemo {
    task_id: i32,
    user_id: i32,
    content: String,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A Memo returns struct")]
impl Memo {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn content(&self) -> String {
        self.content.clone()
    }

    // async fn tasks(&self, context: &Context) -> Vec<Task> {
    //     let task = context.loaders.user_tasks_loader.load(self.id).await;
    //     task.into_iter().map(|t| t.into()).collect()
    // }
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A User update")]
pub struct UpdateMemo {
    content: String,
}

// GraphQLのMemoをdieselのMemoに変換するFromトレイト実装
impl From<memo::Memo> for Memo {
    fn from(memo: memo::Memo) -> Self {
        Self {
            id: memo.id,
            task_id: memo.task_id,
            user_id: memo.user_id,
            content: memo.content,
        }
    }
}

/// GraphQLの構造体NewMemoをdieselの構造体MemoNewFormに変換するFromトレイト実装
impl<'a> TryFrom<&'a NewMemo> for memo::MemoNewForm<'a> {
    type Error = ParseError;
    fn try_from(new_memo: &'a NewMemo) -> Result<Self, Self::Error> {
        Ok(Self {
            task_id: new_memo.task_id,
            user_id: new_memo.user_id,
            content: &new_memo.content,
        })
    }
}

/// GraphQLの構造体UpdateMemoをdieselの構造体MemoUpdateFormに変換するFromトレイト実装
impl<'a> From<&'a UpdateMemo> for memo::MemoUpdateForm<'a> {
    fn from(update_memo: &'a UpdateMemo) -> Self {
        Self {
            content: &update_memo.content,
        }
    }
}
