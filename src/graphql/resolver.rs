use crate::db::repositories::milestone_repository::MilestoneRepository;
use crate::db::repositories::project_repository::ProjectRepository;
use crate::db::repositories::task_repository::TaskRepository;
use crate::db::repositories::user_repository::UserRepository;
use crate::db::repositories::memo_repository::MemoRepository;
use crate::db::repositories::holiday_repository::HolidayRepository;
use crate::graphql::schema::milestone::{Milestone, NewMilestone, UpdateMilestone};
use crate::graphql::schema::project::{NewProject, Project, UpdateProject};
use crate::graphql::schema::task::{NewTask, Task, UpdateTask};
use crate::graphql::schema::user::{NewUser, UpdateUser, User};
use crate::graphql::schema::memo::{NewMemo, UpdateMemo, Memo};
use crate::graphql::schema::holiday::{NewHoliday, Holiday};
use crate::graphql::schema::{Context, Mutation, Query};
use diesel::result::Error;
use juniper::{FieldError, FieldResult};

async fn load_all_users(ctx: &Context) -> Result<Vec<User>, Error> {
    let users = UserRepository::all_users(ctx)?;
    let mut result = Vec::new();
    for user in users {
        ctx.loaders.users_loader.prime(user.id, user.clone()).await;
        result.push(user.into())
    }
    Ok(result)
}

// GraphQLTypeトレイトを勝手に実装してくれるアトリビュート
#[juniper::graphql_object(Context = Context)]
impl Query {
    fn api_version(&self) -> &'static str {
        "1.0"
    }

    /// プロジェクトすべてを取得するクエリ
    fn all_projects(&self, context: &Context) -> FieldResult<Vec<Project>> {
        // and_thenでResultを別種のResultに変換
        // ここではinto_iterでイテレータを取得し、mapで値一つ一つに対するintoを呼び出した値
        // 最後のcollectはイテレータをVecに戻しているだけ
        // Errorの場合はmap_errに入るのでintoで適当な値に変換
        let projects = ProjectRepository::all_projects(context)
            .and_then(|projects| Ok(projects.into_iter().map(|p| p.into()).collect()))
            .map_err(Into::into);
        projects
    }

    /// パラメータで与えられたIDのプロジェクトを取得するクエリ
    async fn find_project(&self, context: &Context, id: i32) -> FieldResult<Project> {
        let project = ProjectRepository::find_project(context, id)?;
        Ok(project.into())
    }

    /// 全ユーザーを取得するクエリ
    async fn all_users(&self, context: &Context) -> FieldResult<Vec<User>> {
        load_all_users(context).await.map_err(Into::into)
    }

    /// プロジェクト内でユーザーに紐づくタスクを取得するクエリ
    fn search_project_tasks(
        &self,
        context: &Context,
        project_id: i32,
        user_id: i32,
    ) -> FieldResult<Vec<Task>> {
        let tasks = TaskRepository::search_tasks_by_user(context, project_id, user_id)
            .and_then(|tasks| Ok(tasks.into_iter().map(|t| t.into()).collect()))
            .map_err(Into::into);
        tasks
    }
    /// プロジェクトすべてを取得するクエリ
    fn all_holidays(&self, context: &Context) -> FieldResult<Vec<Holiday>> {
        // and_thenでResultを別種のResultに変換
        // ここではinto_iterでイテレータを取得し、mapで値一つ一つに対するintoを呼び出した値
        // 最後のcollectはイテレータをVecに戻しているだけ
        // Errorの場合はmap_errに入るのでintoで適当な値に変換
        let holidays = HolidayRepository::all_holidays(context)
            .and_then(|holidays| Ok(holidays.into_iter().map(|h| h.into()).collect()))
            .map_err(Into::into);
        holidays
    }
}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_project(
        &self,
        context: &Context,
        new_project: NewProject,
    ) -> Result<Project, FieldError> {
        let ret = ProjectRepository::insert_project(context, new_project)?;
        Ok(ret.into())
    }

    async fn create_user(&self, context: &Context, new_user: NewUser) -> Result<User, FieldError> {
        let ret = UserRepository::insert_user(context, new_user)?;
        Ok(ret.into())
    }

    async fn create_task(
        &self,
        context: &Context,
        new_task: NewTask,
    ) -> Result<Vec<Task>, FieldError> {
        let ret = TaskRepository::insert_task(context, new_task)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }

    async fn create_milestone(
        &self,
        context: &Context,
        new_milestone: NewMilestone,
    ) -> Result<Vec<Milestone>, FieldError> {
        let ret = MilestoneRepository::insert_milestone(context, new_milestone)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }

    async fn delete_task(
        &self,
        context: &Context,
        task_pkey: i32,
    ) -> Result<Vec<Task>, FieldError> {
        let ret = TaskRepository::delete_task(context, task_pkey)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }

    async fn delete_milestone(
        &self,
        context: &Context,
        milestone_pkey: i32,
    ) -> Result<Vec<Milestone>, FieldError> {
        let ret = MilestoneRepository::delete_milestone(context, milestone_pkey)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }

    async fn batch_create_user(
        &self,
        context: &Context,
        new_users: Vec<NewUser>,
    ) -> Result<Vec<User>, FieldError> {
        let ret = UserRepository::batch_insert_user(context, &new_users)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }

    async fn update_project(
        &self,
        context: &Context,
        id: i32,
        update_project: UpdateProject,
    ) -> Result<Project, FieldError> {
        let ret = ProjectRepository::update_project(context, id, update_project)?;
        Ok(ret.into())
    }

    async fn update_user(
        &self,
        context: &Context,
        id: i32,
        update_user: UpdateUser,
    ) -> Result<User, FieldError> {
        let ret = UserRepository::update_user(context, id, update_user)?;
        Ok(ret.into())
    }

    async fn update_task(
        &self,
        context: &Context,
        id: i32,
        update_task: UpdateTask,
    ) -> Result<Task, FieldError> {
        let ret = TaskRepository::update_task(context, id, update_task)?;
        Ok(ret.into())
    }

    async fn update_milestone(
        &self,
        context: &Context,
        id: i32,
        update_milestone: UpdateMilestone,
    ) -> Result<Milestone, FieldError> {
        let ret = MilestoneRepository::update_milestone(context, id, update_milestone)?;
        Ok(ret.into())
    }

    async fn create_memo(&self, context: &Context, new_memo: NewMemo) -> Result<Vec<Memo>, FieldError> {
        let ret = MemoRepository::insert_memo(context, new_memo)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }

    async fn delete_memo(&self, context: &Context, id: i32) -> Result<Vec<Memo>, FieldError> {
        let ret = MemoRepository::delete_memo(context, id)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }

    async fn create_holiday(&self, context: &Context, new_holiday: NewHoliday) -> Result<Holiday, FieldError> {
        let ret = HolidayRepository::insert_holiday(context, new_holiday)?;
        Ok(ret.into())
    }

    async fn delete_holiday(&self, context: &Context, id: i32) -> Result<Vec<Holiday>, FieldError> {
        let ret = HolidayRepository::delete_holiday(context, id)?;
        Ok(ret.into_iter().map(Into::into).collect())
    }
}
