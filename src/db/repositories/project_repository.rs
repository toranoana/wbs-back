use self::diesel::prelude::*;
use crate::db::models::project::{ProjectNewForm, ProjectUpdateForm};
use crate::db::{
    manager::DataPgPool, models::project_with_task_duration_view::ProjectWithTaskDurationView,
};
use crate::graphql::schema::project::{NewProject, UpdateProject};
use crate::graphql::schema::Context;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::result::Error;
use log::debug;
use std::convert::TryInto;

extern crate diesel;

pub struct ProjectRepository {}

/// TODO: 各メソッドの引数をcontextからDataPgPoolに統一したい
impl ProjectRepository {
    pub fn all_projects(context: &Context) -> Result<Vec<ProjectWithTaskDurationView>, Error> {
        use crate::view_schema::project_with_task_duration_view::dsl::*;
        let conn = &context.pool.get().unwrap();
        project_with_task_duration_view.load(conn)
    }

    pub fn any_projects(
        pool: &DataPgPool,
        keys: &[i32],
    ) -> Result<Vec<ProjectWithTaskDurationView>, Error> {
        use crate::view_schema::project_with_task_duration_view::dsl::*;
        let conn = &pool.get().unwrap();
        let select_query = project_with_task_duration_view.filter(id.eq_any(keys));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.get_results::<ProjectWithTaskDurationView>(conn)
    }

    // TODO: ここのエラーが不格好なので直したい(failure使うとか？)
    pub fn insert_project(
        context: &Context,
        new_project: NewProject,
    ) -> Result<ProjectWithTaskDurationView, Box<dyn std::error::Error>> {
        use crate::schema::projects::dsl::*;
        use diesel::dsl::insert_into;
        let conn = &context.pool.get().unwrap();
        // PhotoFormのメンバは参照値なので参照値でintoかつライフタイムに注意
        let project_form: ProjectNewForm = (&new_project).try_into()?;
        let rows_inserted = insert_into(projects)
            .values(&project_form)
            .returning(id)
            .get_result(conn)?;
        // TODO: updateと合わせて結果の取得方法がちょっと変
        Ok(ProjectRepository::find_project(context, rows_inserted)?)
    }

    pub fn update_project(
        context: &Context,
        pkey: i32,
        update_project: UpdateProject,
    ) -> Result<ProjectWithTaskDurationView, Box<dyn std::error::Error>> {
        use crate::schema::projects::dsl::*;
        use diesel::dsl::update;
        let conn = &context.pool.get().unwrap();
        let project_form: ProjectUpdateForm = (&update_project).try_into()?;
        let rows_inserted = update(projects.filter(id.eq(pkey)))
            .set(&project_form)
            .returning(id)
            .get_result(conn)?;
        Ok(ProjectRepository::find_project(context, rows_inserted)?)
    }

    pub fn find_project(
        context: &Context,
        pkey: i32,
    ) -> Result<ProjectWithTaskDurationView, Error> {
        use crate::view_schema::project_with_task_duration_view::dsl::*;
        let conn = &context.pool.get().unwrap();
        let select_query = project_with_task_duration_view.find(pkey).select((
            id,
            title,
            color,
            is_archived,
            started_at,
            ended_at,
            min_started_at,
            max_ended_at,
        ));
        let sql = debug_query::<Pg, _>(&select_query).to_string();
        debug!("{:?}", sql);
        select_query.first(conn)
    }
}
