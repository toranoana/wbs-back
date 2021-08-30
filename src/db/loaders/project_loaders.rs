use crate::db::manager::DataPgPool;
use crate::db::{
    models::project_with_task_duration_view::ProjectWithTaskDurationView,
    repositories::project_repository::ProjectRepository,
};
use async_trait::async_trait;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use log::error;
use std::collections::HashMap;

extern crate diesel;

pub struct ProjectsLoadFn {
    /// 非同期関数内でコネクションプールを直接使おうとすると怒られるのでDataをそのまま持ち回す
    pub pool: DataPgPool,
}

impl ProjectsLoadFn {
    pub fn projects(&self, keys: &[i32]) -> Vec<ProjectWithTaskDurationView> {
        let result = ProjectRepository::any_projects(&self.pool, keys);
        match result {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                Vec::new()
            }
        }
    }
}

#[async_trait]
impl BatchFn<i32, ProjectWithTaskDurationView> for ProjectsLoadFn {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, ProjectWithTaskDurationView> {
        let res = self.projects(keys);
        // associationを取るためには構造体のUserが必要なのでidからダミーを作成
        res.iter().map(|p| (p.id, p.clone())).collect()
    }
}

pub type ProjectsLoader = Loader<i32, ProjectWithTaskDurationView, ProjectsLoadFn>;

pub fn create_projects_loader(pool: &DataPgPool) -> ProjectsLoader {
    Loader::new(ProjectsLoadFn { pool: pool.clone() }).with_yield_count(100)
}
