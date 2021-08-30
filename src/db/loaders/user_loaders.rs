use crate::db::manager::DataPgPool;
use crate::db::{models::user::User, repositories::user_repository::UserRepository};
use async_trait::async_trait;
use dataloader::cached::Loader;
use dataloader::BatchFn;
use log::error;
use std::collections::HashMap;

extern crate diesel;

pub struct UsersLoadFn {
    /// 非同期関数内でコネクションプールを直接使おうとすると怒られるのでDataをそのまま持ち回す
    pub pool: DataPgPool,
}

impl UsersLoadFn {
    pub fn users(&self, keys: &[i32]) -> Vec<User> {
        let result = UserRepository::any_users(&self.pool, keys);
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
impl BatchFn<i32, User> for UsersLoadFn {
    async fn load(&self, keys: &[i32]) -> HashMap<i32, User> {
        let res = self.users(keys);
        // associationを取るためには構造体のUserが必要なのでidからダミーを作成
        res.iter().map(|u| (u.id, u.clone())).collect()
    }
}

pub type UsersLoader = Loader<i32, User, UsersLoadFn>;

pub fn create_users_loader(pool: &DataPgPool) -> UsersLoader {
    Loader::new(UsersLoadFn { pool: pool.clone() }).with_yield_count(100)
}
