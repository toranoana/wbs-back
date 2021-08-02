use self::diesel::prelude::*;
use crate::db::manager::DataPgPool;
use crate::db::models::milestone::Milestone;
use crate::db::models::project::AssocProject;
use crate::db::repositories::milestone_repository::MilestoneRepository;
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

pub struct ProjectMilestonesLoadFn {
    /// 非同期関数内でコネクションプールを直接使おうとすると怒られるのでDataをそのまま持ち回す
    pub pool: DataPgPool,
}

impl ProjectMilestonesLoadFn {
    pub fn project_milestones(&self, keys: &[i32]) -> Vec<Milestone> {
        let query_result = MilestoneRepository::any_project_milestones(&self.pool, keys);
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
impl BatchFn<i32, Vec<Milestone>> for ProjectMilestonesLoadFn {
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Vec<Milestone>> {
        let assoc_projects: Vec<AssocProject> = create_assoc_projects(keys.to_vec());
        let project_milestones: Vec<Vec<Milestone>> =
            self.project_milestones(keys).grouped_by(&assoc_projects);
        let result = assoc_projects
            .iter()
            .zip(project_milestones)
            .map(|assoc| (assoc.0.id, assoc.1.clone()))
            .collect();
        result
    }
}

pub type ProjectMilestonesLoader = Loader<i32, Vec<Milestone>, ProjectMilestonesLoadFn>;

pub fn create_project_milestones_loader(pool: &DataPgPool) -> ProjectMilestonesLoader {
    Loader::new(ProjectMilestonesLoadFn { pool: pool.clone() }).with_yield_count(100)
}
