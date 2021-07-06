use crate::db::loaders::milestone_loaders::{
    create_project_milestones_loader, ProjectMilestonesLoader,
};
use crate::db::loaders::task_loaders::{
    create_project_tasks_loader, create_user_tasks_loader, ProjectTasksLoader, UserTasksLoader,
};
use crate::db::loaders::user_loaders::{create_users_loader, UsersLoader};
use crate::db::manager::DataPgPool;
use project_loaders::{create_projects_loader, ProjectsLoader};

pub mod milestone_loaders;
pub mod project_loaders;
pub mod task_loaders;
pub mod user_loaders;

pub struct Loaders {
    pub user_tasks_loader: UserTasksLoader,
    pub project_tasks_loader: ProjectTasksLoader,
    pub users_loader: UsersLoader,
    pub projects_loader: ProjectsLoader,
    pub project_milestones_loader: ProjectMilestonesLoader,
}

impl Loaders {
    pub fn new(pool: &DataPgPool) -> Loaders {
        Loaders {
            user_tasks_loader: create_user_tasks_loader(pool),
            project_tasks_loader: create_project_tasks_loader(pool),
            users_loader: create_users_loader(pool),
            projects_loader: create_projects_loader(pool),
            project_milestones_loader: create_project_milestones_loader(pool),
        }
    }
}
