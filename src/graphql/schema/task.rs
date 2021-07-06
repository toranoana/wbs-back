use crate::db::models::*;
use crate::graphql::schema::project::Project;
use crate::graphql::schema::user::User;
use crate::graphql::schema::memo::Memo;
use crate::graphql::schema::Context;
use crate::db::repositories::memo_repository::MemoRepository;
use chrono::{Local, NaiveDate, ParseError};
use juniper::{FieldResult, ID};
use std::convert::TryFrom;

pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
    pub task_name: String,
    pub started_at: String,
    pub ended_at: String,
    pub progress: i16,
    pub order_number: f64,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Task create")]
pub struct NewTask {
    project_id: i32,
    user_id: i32,
    task_name: String,
    started_at: String,
    ended_at: String,
    order_number: Option<f64>,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Task update")]
pub struct UpdateTask {
    user_id: Option<i32>,
    task_name: Option<String>,
    started_at: Option<String>,
    ended_at: Option<String>,
    // TODO: バリデーションこっち側にいれる場合にはprogressはi16の範囲に収まるように(実際は0~100だけど)
    progress: Option<i32>,
    order_number: Option<f64>,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A Task returns struct")]
impl Task {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn task_name(&self) -> String {
        self.task_name.clone()
    }

    fn started_at(&self) -> String {
        self.started_at.clone()
    }

    fn ended_at(&self) -> String {
        self.ended_at.clone()
    }

    fn progress(&self) -> i32 {
        self.progress as i32
    }

    fn order_number(&self) -> f64 {
        self.order_number
    }

    async fn project(&self, context: &Context) -> Project {
        let project = context.loaders.projects_loader.load(self.project_id).await;
        project.into()
    }

    async fn user(&self, context: &Context) -> User {
        let user = context.loaders.users_loader.load(self.user_id).await;
        user.into()
    }

    async fn memos(&self, context: &Context) -> FieldResult<Vec<Memo>> {
        let memos = MemoRepository::task_memos(context, self.id);
        match memos {
            Ok(m) => Ok(m.into_iter().map(|memo| memo.into()).collect()),
            Err(_) => Ok(Vec::new())
        }
    }
}

/// GraphQLのProjectをdieselのProjectに変換するメソッド
impl From<task::Task> for Task {
    fn from(task: task::Task) -> Self {
        Self {
            id: task.id,
            // 以下２つダミー
            project_id: task.project_id,
            user_id: task.user_id,
            task_name: task.task_name,
            started_at: task.started_at.format("%Y-%m-%d").to_string(),
            ended_at: task.ended_at.format("%Y-%m-%d").to_string(),
            progress: task.progress,
            order_number: task.order_number,
        }
    }
}

impl<'a> TryFrom<&'a NewTask> for task::TaskNewForm<'a> {
    type Error = ParseError;

    fn try_from(new_task: &'a NewTask) -> Result<Self, Self::Error> {
        let start =
            NaiveDate::parse_from_str(&new_task.started_at, "%Y-%m-%d")?.and_hms(00, 00, 00);
        let end = NaiveDate::parse_from_str(&new_task.ended_at, "%Y-%m-%d")?.and_hms(23, 59, 59);
        Ok(Self {
            project_id: new_task.project_id,
            user_id: new_task.user_id,
            task_name: &new_task.task_name,
            started_at: start,
            ended_at: end,
            progress: 0,
            order_number: new_task.order_number,
            created_at: Local::now().naive_local(),
            updated_at: Local::now().naive_local(),
        })
    }
}

impl<'a> TryFrom<&'a UpdateTask> for task::TaskUpdateForm<'a> {
    type Error = ParseError;

    fn try_from(update_task: &'a UpdateTask) -> Result<Self, Self::Error> {
        let start = match &update_task.started_at {
            Some(start) => Some(NaiveDate::parse_from_str(&start, "%Y-%m-%d")?.and_hms(00, 00, 00)),
            None => None,
        };
        let end = match &update_task.ended_at {
            Some(end) => Some(NaiveDate::parse_from_str(&end, "%Y-%m-%d")?.and_hms(23, 59, 59)),
            None => None,
        };

        let progress = match update_task.progress {
            Some(progress) => Some(i16::try_from(progress).expect("progress number is too large.")),
            None => None,
        };

        Ok(Self {
            user_id: update_task.user_id,
            task_name: update_task.task_name.as_ref().map(AsRef::as_ref),
            started_at: start,
            ended_at: end,
            progress,
            order_number: update_task.order_number,
            updated_at: Local::now().naive_local(),
        })
    }
}
