use self::diesel::prelude::*;
use crate::db::manager::DataPgPool;
use crate::db::models::project::AssocProject;
use crate::db::models::task::Task;
use crate::db::{models::user::AssocUser, repositories::task_repository::TaskRepository};
use async_trait::async_trait;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use log::error;
use std::collections::HashMap;

extern crate diesel;

/// Association用のProjectレコードベクタを作る関数
fn create_assoc_projects(keys: Vec<i32>) -> Vec<AssocProject> {
    keys.into_iter().map(|k| AssocProject { id: k }).collect()
}

/// Association用のUserレコードベクタを作る関数
fn create_assoc_users(keys: Vec<i32>) -> Vec<AssocUser> {
    keys.into_iter().map(|k| AssocUser { id: k }).collect()
}

pub struct UserTasksLoadFn {
    /// 非同期関数内でコネクションプールを直接使おうとすると怒られるのでDataをそのまま持ち回す
    pub pool: DataPgPool,
}

impl UserTasksLoadFn {
    pub fn user_tasks(&self, keys: &[i32]) -> Vec<Task> {
        let query_result = TaskRepository::any_user_tasks(&self.pool, keys);
        match query_result {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                Vec::new()
            }
        }
    }
}

#[async_trait]
impl BatchFn<i32, Vec<Task>> for UserTasksLoadFn {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Vec<Task>> {
        let assoc_users: Vec<AssocUser> = create_assoc_users(keys.to_vec());
        let user_tasks: Vec<Vec<Task>> = self.user_tasks(keys).grouped_by(&assoc_users);
        let result = assoc_users
            .iter()
            .zip(user_tasks)
            .map(|assoc| (assoc.0.id, assoc.1.clone()))
            .collect();
        result
    }
}

pub struct ProjectTasksLoadFn {
    /// 非同期関数内でコネクションプールを直接使おうとすると怒られるのでDataをそのまま持ち回す
    pub pool: DataPgPool,
}

impl ProjectTasksLoadFn {
    pub fn project_tasks(&self, keys: &[i32]) -> Vec<Task> {
        let query_result = TaskRepository::any_project_tasks(&self.pool, keys);
        match query_result {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                Vec::new()
            }
        }
    }
}

#[async_trait]
impl BatchFn<i32, Vec<Task>> for ProjectTasksLoadFn {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, Vec<Task>> {
        // associationを取るためには構造体のUserが必要なのでidからダミーを作成
        let assoc_projects: Vec<AssocProject> = create_assoc_projects(keys.to_vec());
        let project_tasks = self.project_tasks(keys).grouped_by(&assoc_projects);
        let result = assoc_projects
            .iter()
            .zip(project_tasks)
            .map(|assoc| (assoc.0.id, assoc.1.clone()))
            .collect();
        result
    }
}

pub type UserTasksLoader = Loader<i32, Vec<Task>, UserTasksLoadFn>;
pub type ProjectTasksLoader = Loader<i32, Vec<Task>, ProjectTasksLoadFn>;

pub fn create_user_tasks_loader(pool: &DataPgPool) -> UserTasksLoader {
    Loader::new(UserTasksLoadFn { pool: pool.clone() }).with_yield_count(100)
}

pub fn create_project_tasks_loader(pool: &DataPgPool) -> ProjectTasksLoader {
    Loader::new(ProjectTasksLoadFn { pool: pool.clone() }).with_yield_count(100)
}
