use self::diesel::prelude::*;
use crate::db::{
    manager::DataPgPool,
    models::task::{Task, TaskNewForm, TaskUpdateForm},
};
use crate::graphql::schema::task::{NewTask, UpdateTask};
use crate::graphql::schema::Context;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::result::Error;
use log::debug;
use std::convert::TryInto;

extern crate diesel;

pub struct TaskRepository {}

impl TaskRepository {
    pub fn all_tasks(context: &Context) -> Result<Vec<Task>, Error> {
        use crate::schema::tasks::dsl::*;
        let conn = &context.pool.get().unwrap();
        tasks.load::<Task>(conn)
    }

    pub fn find_tasks(pool: &DataPgPool, keys: Vec<i32>) -> Result<Vec<Task>, Error> {
        use crate::schema::tasks::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = tasks.filter(id.eq_any(keys));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.get_results::<Task>(conn)
    }

    pub fn search_tasks_by_user(context: &Context, project_key: i32, user_key: i32) -> Result<Vec<Task>, Error> {
        use crate::schema::tasks::dsl::*;
        let conn = &context.pool.get().unwrap();
        let select_query = tasks.filter(project_id.eq(project_key)).filter(user_id.eq(user_key));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.get_results::<Task>(conn)
    }

    // TODO: ここのエラーが不格好なので直したい(failure使うとか？)
    pub fn insert_task(
        context: &Context,
        new_task: NewTask,
    ) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        use crate::schema::tasks::dsl::*;
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let task_form: TaskNewForm = (&new_task).try_into()?;
        let rows_inserted = insert_into(tasks)
            .values(&task_form)
            .get_result(conn)
            .and_then(|t: Task| tasks.filter(project_id.eq(t.project_id)).load::<Task>(conn))?;
        Ok(rows_inserted)
    }

    pub fn batch_insert_task(
        pool: DataPgPool,
        new_tasks: &[TaskNewForm],
    ) -> Result<Vec<Task>, Error> {
        use crate::schema::tasks::dsl::tasks;
        use diesel::dsl::insert_into;
        let conn = &pool.get().unwrap();
        let rows_inserted = insert_into(tasks).values(new_tasks).get_results(conn)?;
        Ok(rows_inserted)
    }

    pub fn delete_task(
        context: &Context,
        task_pkey: i32,
    ) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        use crate::schema::tasks::dsl::*;
        use diesel::dsl::delete;
        let conn = &context.pool.get().unwrap();
        let project_pkey = tasks
            .find(task_pkey)
            .select(project_id)
            .get_result::<i32>(conn)?;

        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let rows_deleted = delete(tasks.filter(id.eq(task_pkey)))
            .execute(conn)
            .and_then(|_| tasks.filter(project_id.eq(project_pkey)).load::<Task>(conn))?;
        Ok(rows_deleted)
    }

    pub fn update_task(
        context: &Context,
        pkey: i32,
        update_task: UpdateTask,
    ) -> Result<Task, Box<dyn std::error::Error>> {
        use crate::schema::tasks::dsl::*;
        use diesel::dsl::update;
        let conn = &context.pool.get().unwrap();
        let task_form: TaskUpdateForm = (&update_task).try_into()?;
        // TODO: 他の場所と同じだけどfindでよい
        let rows_inserted = update(tasks.filter(id.eq(pkey)))
            .set(&task_form)
            .get_result(conn)?;
        Ok(rows_inserted)
    }

    pub fn any_project_tasks(pool: &DataPgPool, project_pkeys: &[i32]) -> Result<Vec<Task>, Error> {
        use crate::schema::tasks::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = tasks.filter(project_id.eq_any(project_pkeys)).order((order_number));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.load::<Task>(conn)
    }

    pub fn any_user_tasks(pool: &DataPgPool, user_pkeys: &[i32]) -> Result<Vec<Task>, Error> {
        use crate::schema::tasks::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = tasks.filter(user_id.eq_any(user_pkeys));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.load::<Task>(conn)
    }
}
