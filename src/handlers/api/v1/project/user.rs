use crate::db::manager::DataPgPool;
use crate::graphql::schema::Schema;
use actix_web::{web, Error, HttpResponse};
use juniper::http::GraphQLRequest;
use log::debug;
use std::sync::Arc;

pub async fn import(pool: DataPgPool) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}
