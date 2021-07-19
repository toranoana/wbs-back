use self::diesel::prelude::*;
use crate::db::{
    models::holiday::{Holiday},
};
use crate::graphql::schema::Context;
use diesel::result::Error;

extern crate diesel;

pub struct HolidayRepository {}

impl HolidayRepository {

    pub fn all_holidays(context: &Context) -> Result<Vec<Holiday>, Error> {
        use crate::schema::holydays::dsl::*;
        let conn = &context.pool.get().unwrap();
        // 構造体をばらしたtupleでもロードは可能
        holidays.load::<Holiday>(conn)
    }
}
