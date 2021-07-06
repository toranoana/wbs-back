use crate::db::models::*;
use crate::graphql::schema::project::Project;
use crate::graphql::schema::Context;
use chrono::{Local, NaiveDate, ParseError};
use juniper::ID;
use std::convert::TryFrom;

pub struct Milestone {
    pub id: i32,
    pub project_id: i32,
    pub description: String,
    pub confirmed_at: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Milestone create")]
pub struct NewMilestone {
    project_id: i32,
    confirmed_at: String,
    description: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Milestone update")]
pub struct UpdateMilestone {
    confirmed_at: Option<String>,
    description: Option<String>,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A Milestone returns struct")]
impl Milestone {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn confirmed_at(&self) -> String {
        self.confirmed_at.clone()
    }

    async fn project(&self, context: &Context) -> Project {
        let project = context.loaders.projects_loader.load(self.project_id).await;
        project.into()
    }
}

/// GraphQLのProjectをdieselのProjectに変換するメソッド
impl From<milestone::Milestone> for Milestone {
    fn from(milestone: milestone::Milestone) -> Self {
        Self {
            id: milestone.id,
            project_id: milestone.project_id,
            description: milestone.description,
            confirmed_at: milestone.confirmed_at.format("%Y-%m-%d").to_string(),
        }
    }
}

impl<'a> TryFrom<&'a NewMilestone> for milestone::MilestoneNewForm<'a> {
    type Error = ParseError;

    fn try_from(new_milestone: &'a NewMilestone) -> Result<Self, Self::Error> {
        let confirmed =
            NaiveDate::parse_from_str(&new_milestone.confirmed_at, "%Y-%m-%d")?.and_hms(00, 00, 00);
        Ok(Self {
            project_id: new_milestone.project_id,
            description: &new_milestone.description,
            confirmed_at: confirmed,
        })
    }
}

impl<'a> TryFrom<&'a UpdateMilestone> for milestone::MilestoneUpdateForm<'a> {
    type Error = ParseError;

    fn try_from(update_milestone: &'a UpdateMilestone) -> Result<Self, Self::Error> {
        let confirmed = match &update_milestone.confirmed_at {
            Some(date) => Some(NaiveDate::parse_from_str(&date, "%Y-%m-%d")?.and_hms(00, 00, 00)),
            None => None,
        };

        Ok(Self {
            description: update_milestone.description.as_ref().map(AsRef::as_ref),
            confirmed_at: confirmed,
            updated_at: Local::now().naive_local(),
        })
    }
}
