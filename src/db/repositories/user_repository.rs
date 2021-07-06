use self::diesel::prelude::*;
use crate::db::{
    manager::DataPgPool,
    models::user::{User, UserNewForm, UserUpdateForm},
};
use crate::graphql::schema::user::{NewUser, UpdateUser};
use crate::graphql::schema::Context;
use crate::schema::users::dsl::*;
use chrono::ParseError;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::result::Error;
use log::debug;
use std::convert::TryInto;

extern crate diesel;

pub struct UserRepository;

const USER_SELECTIONS: (id, display_name, is_disabled, created_at, updated_at) =
    (id, display_name, is_disabled, created_at, updated_at);

/// TODO: とりあえずstaticメソッドにしたけど関数にするべきかも
impl UserRepository {
    pub fn all_users(context: &Context) -> Result<Vec<User>, Error> {
        let conn = &context.pool.get().unwrap();
        // 構造体をばらしたtupleでもロードは可能
        users.load::<User>(conn)
    }

    pub fn any_users(pool: &DataPgPool, keys: &[i32]) -> Result<Vec<User>, Error> {
        use crate::schema::users::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = users.filter(id.eq_any(keys));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.get_results::<User>(conn)
    }

    // TODO: ここのエラーが不格好なので直したい(failure使うとか？)
    pub fn insert_user(
        context: &Context,
        new_user: NewUser,
    ) -> Result<User, Box<dyn std::error::Error>> {
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let user_form: UserNewForm = (&new_user).try_into()?;
        let rows_inserted = insert_into(users).values(&user_form).get_result(conn)?;
        Ok(rows_inserted)
    }

    pub fn batch_insert_user(
        context: &Context,
        new_users: &[NewUser],
    ) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let user_forms: Vec<UserNewForm> = new_users
            .iter()
            .map(TryInto::try_into)
            .collect::<Result<Vec<UserNewForm>, ParseError>>()?;
        let rows_inserted = insert_into(users).values(&user_forms).get_results(conn)?;
        Ok(rows_inserted)
    }

    pub fn update_user(
        context: &Context,
        pkey: i32,
        update_user: UpdateUser,
    ) -> Result<User, Error> {
        use diesel::dsl::update;
        let conn = &context.pool.get().unwrap();
        let user_form: UserUpdateForm = (&update_user).into();
        let rows_inserted = update(users.filter(id.eq(pkey)))
            .set(&user_form)
            .get_result(conn)?;
        Ok(rows_inserted)
    }

    pub fn find_user(context: &Context, pkey: i32) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let conn = &context.pool.get().unwrap();
        let select_query = users
            .find(pkey)
            .filter(is_disabled.eq(false))
            .select(USER_SELECTIONS);
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.first(conn)
    }
}
