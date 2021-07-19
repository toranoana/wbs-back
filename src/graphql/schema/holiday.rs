use crate::db::models::*;
use crate::graphql::schema::Context;
use chrono::{NaiveDate, ParseError};
use juniper::ID;
use std::convert::TryFrom;

pub struct Holiday {
    id: i32,
    holiday_name: String,
    target_at: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Holiday create")]
pub struct NewHoliday {
    holiday_name: String,
    target_at: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A Holiday update")]
pub struct UpdateHoliday {
    holiday_name: Option<String>,
    target_at: Option<String>,
}

#[juniper::graphql_object(Context = Context)]
#[graphql(description = "A Holiday returns struct")]
impl Holiday {
    fn id(&self) -> ID {
        ID::new(self.id.to_string())
    }

    fn holiday_name(&self) -> String {
        self.holiday_name.clone()
    }

    fn target_at(&self) -> String {
        self.target_at.clone()
    }

}


// GraphQLのMemoをdieselのMemoに変換するFromトレイト実装
impl From<holiday::Holiday> for Holiday {
    fn from(holiday: holiday::Holiday) -> Self {
        Self {
            id: holiday.id,
            holiday_name: holiday.holiday_name,
            target_at: holiday.target_at.format("%Y-%m-%d").to_string(),
        }
    }
}

/// GraphQLの構造体NewMemoをdieselの構造体MemoNewFormに変換するFromトレイト実装
impl<'a> TryFrom<&'a NewHoliday> for holiday::HolidayNewForm<'a> {
    type Error = ParseError;
    fn try_from(new_holiday: &'a NewHoliday) -> Result<Self, Self::Error> {

        let target =
            NaiveDate::parse_from_str(&new_holiday.target_at, "%Y-%m-%d")?.and_hms(00, 00, 00);
        Ok(Self {
            holiday_name: &new_holiday.holiday_name,
            target_at: target
        })
    }
}

/// GraphQLの構造体UpdateMemoをdieselの構造体MemoUpdateFormに変換するFromトレイト実装
impl<'a> TryFrom<&'a UpdateHoliday> for holiday::HolidayUpdateForm<'a> {
    type Error = ParseError;

    fn try_from(update_holiday: &'a UpdateHoliday) -> Result<Self, Self::Error> {

        let target = match &update_holiday.target_at {
            Some(target) => Some(NaiveDate::parse_from_str(&target, "%Y-%m-%d")?.and_hms(00, 00, 00)),
            None => None,
        };
        Ok(Self {
            holiday_name: update_holiday.holiday_name.as_ref().map(AsRef::as_ref),
            target_at: target
        })
    }
}
