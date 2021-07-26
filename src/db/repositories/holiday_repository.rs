use self::diesel::prelude::*;
use crate::db::{
    models::holiday::{Holiday},
    models::holiday::{HolidayNewForm},
};
use crate::graphql::schema::holiday::{NewHoliday, UpdateHoliday};
use crate::graphql::schema::Context;
use diesel::result::Error;
use std::convert::TryInto;

extern crate diesel;

pub struct HolidayRepository {}

impl HolidayRepository {

    pub fn all_holidays(context: &Context) -> Result<Vec<Holiday>, Error> {
        use crate::schema::holidays::dsl::*;
        let conn = &context.pool.get().unwrap();
        // 構造体をばらしたtupleでもロードは可能
        holidays.load::<Holiday>(conn)
    }

    pub fn insert_holiday(
        context: &Context,
        new_holiday: NewHoliday,
    ) -> Result<Holiday, Box<dyn std::error::Error>> {
        use crate::schema::holidays::dsl::*;
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let holiday_form: HolidayNewForm = (&new_holiday).try_into()?;
        let rows_inserted = insert_into(holidays)
            .values(&holiday_form)
            .get_result(conn)?;
        Ok(rows_inserted)
    }

    pub fn delete_holiday(
        context: &Context,
        holiday_pkey: i32,
    ) -> Result<Vec<Holiday>, Box<dyn std::error::Error>> {
        use crate::schema::holidays::dsl::*;
        use diesel::dsl::delete;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let rows_deleted = delete(holidays.filter(id.eq(holiday_pkey)))
            .execute(conn)
            .and_then(|_| holidays.load::<Holiday>(conn))?;
        Ok(rows_deleted)
    }
}
