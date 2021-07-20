use crate::db::models::project_with_task_duration_view::ProjectWithTaskDurationView;
use crate::db::models::*;
use crate::db::repositories::task_repository::TaskRepository;
use crate::graphql::schema::milestone::Milestone;
use crate::graphql::schema::task::Task;
use crate::graphql::schema::Context;
use chrono::{Local, NaiveDate, ParseError};
use core::cmp;
use juniper::{FieldResult, ID};
use std::convert::TryFrom;

#[derive(Default, Debug)]
pub struct Project {
    id: i32,
    title: String,
    color: String,
    is_archived: bool,
    started_at: String,
    ended_at: String,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A Project returns struct")]
impl Project {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn color(&self) -> String {
        self.color.clone()
    }

    fn is_archived(&self) -> bool {
        self.is_archived
    }

    fn started_at(&self) -> String {
        self.started_at.clone()
    }

    fn ended_at(&self) -> String {
        self.ended_at.clone()
    }

    async fn tasks(&self, context: &Context, user_id: Option<i32>) -> FieldResult<Vec<Task>> {
        match user_id {
            Some(u) => TaskRepository::search_tasks_by_user(context, self.id, u)
                .and_then(|tasks| Ok(tasks.into_iter().map(|t| t.into()).collect()))
                .map_err(Into::into),
            None => Ok(context
                .loaders
                .project_tasks_loader
                .load(self.id)
                .await
                .into_iter()
                .map(|t| t.into())
                .collect()),
        }
    }

    async fn milestones(&self, context: &Context) -> FieldResult<Vec<Milestone>> {
        Ok(context
            .loaders
            .project_milestones_loader
            .load(self.id)
            .await
            .into_iter()
            .map(|m| m.into())
            .collect())
    }
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Project create")]
pub struct NewProject {
    title: String,
    color: String,
    started_at: String,
    ended_at: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Project update")]
pub struct UpdateProject {
    title: Option<String>,
    color: Option<String>,
    is_archived: Option<bool>,
    started_at: Option<String>,
    ended_at: Option<String>,
}

impl From<project::Project> for Project {
    fn from(project: project::Project) -> Self {
        Self {
            id: project.id,
            title: project.title,
            color: project.color,
            is_archived: project.is_archived,
            started_at: project.started_at.format("%Y-%m-%d").to_string(),
            ended_at: project.ended_at.format("%Y-%m-%d").to_string(),
        }
    }
}

impl From<ProjectWithTaskDurationView> for Project {
    fn from(project: ProjectWithTaskDurationView) -> Self {
        let started_at = match project.min_started_at {
            Some(t) => cmp::min(t, project.started_at),
            None => project.started_at,
        };
        let ended_at = match project.max_ended_at {
            Some(t) => cmp::max(t, project.ended_at),
            None => project.ended_at,
        };
        Self {
            id: project.id,
            title: project.title,
            color: project.color,
            is_archived: project.is_archived,
            started_at: started_at.format("%Y-%m-%d").to_string(),
            ended_at: ended_at.format("%Y-%m-%d").to_string(),
        }
    }
}

impl<'a> TryFrom<&'a NewProject> for project::ProjectNewForm<'a> {
    type Error = ParseError;

    fn try_from(new_project: &'a NewProject) -> Result<Self, Self::Error> {
        let start =
            NaiveDate::parse_from_str(&new_project.started_at, "%Y-%m-%d")?.and_hms(00, 00, 00);
        let end = NaiveDate::parse_from_str(&new_project.ended_at, "%Y-%m-%d")?.and_hms(23, 59, 59);
        Ok(Self {
            title: &new_project.title,
            color: &new_project.color,
            // TODO: パースする文字列にタイムゾーン情報が必要になるのでNaiveのほうに合わせるべきかも
            started_at: start,
            ended_at: end,
        })
    }
}

impl<'a> TryFrom<&'a UpdateProject> for project::ProjectUpdateForm<'a> {
    type Error = ParseError;

    fn try_from(update_project: &'a UpdateProject) -> Result<Self, Self::Error> {
        let start = match &update_project.started_at {
            Some(start) => Some(NaiveDate::parse_from_str(&start, "%Y-%m-%d")?.and_hms(00, 00, 00)),
            None => None,
        };
        let end = match &update_project.ended_at {
            Some(end) => Some(NaiveDate::parse_from_str(&end, "%Y-%m-%d")?.and_hms(23, 59, 59)),
            None => None,
        };

        Ok(Self {
            title: update_project.title.as_ref().map(AsRef::as_ref),
            color: update_project.color.as_ref().map(AsRef::as_ref),
            is_archived: update_project.is_archived,
            started_at: start,
            ended_at: end,
            updated_at: Local::now().naive_local(),
        })
    }
}
