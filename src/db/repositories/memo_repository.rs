use self::diesel::prelude::*;
use crate::db::{
    manager::DataPgPool,
    models::memo::{Memo, MemoNewForm, MemoUpdateForm},
};
use crate::graphql::schema::memo::{NewMemo, UpdateMemo};
use crate::graphql::schema::Context;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::result::Error;
use log::debug;
use std::convert::TryInto;

extern crate diesel;

pub struct MemoRepository {}

impl MemoRepository {
    // TODO: ここのエラーが不格好なので直したい(failure使うとか？)
    pub fn insert_memo(
        context: &Context,
        new_memo: NewMemo,
    ) -> Result<Vec<Memo>, Box<dyn std::error::Error>> {
        use crate::schema::memos::dsl::*;
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let memo_form: MemoNewForm = (&new_memo).try_into()?;
        let rows_inserted = insert_into(memos)
            .values(&memo_form)
            .get_result(conn)
            .and_then(|memo: Memo| memos.filter(task_id.eq(memo.task_id)).load::<Memo>(conn))?;
        Ok(rows_inserted)
    }

    pub fn delete_memo(
        context: &Context,
        memo_pkey: i32,) -> Result<Vec<Memo>, Box<dyn std::error::Error>> {
            use crate::schema::memos::dsl::*;
            use diesel::dsl::delete;
            let conn = &context.pool.get().unwrap();
            let task_pkey = memos
                .find(memo_pkey)
                .select(task_id)
                .get_result::<i32>(conn)?;
    
            // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
            let rows_deleted = delete(memos.filter(id.eq(memo_pkey)))
                .execute(conn)
                .and_then(|_| memos.filter(task_id.eq(task_pkey)).load::<Memo>(conn))?;
            Ok(rows_deleted)

        }

    pub fn task_memos(context: &Context, task_key: i32) -> Result<Vec<Memo>, Error> {
        use crate::schema::memos::dsl::*;
        let conn = &context.pool.get().unwrap();
        let select_query = memos.filter(task_id.eq(task_key));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.load::<Memo>(conn)
    }

}
