use crate::db::loaders::Loaders;
use crate::db::manager::DataPgPool;
use juniper::EmptySubscription;

pub mod milestone;
pub mod project;
pub mod task;
pub mod user;
pub mod memo;
pub mod holiday;

pub struct Context {
    pub pool: DataPgPool,
    pub loaders: Loaders,
}

impl juniper::Context for Context {}

pub struct Query;
pub struct Mutation;
// pub type Schema = juniper::RootNode<'static, Query, Mutation>;
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::<Context>::new())
}
