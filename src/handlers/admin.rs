use crate::db::manager::DataPgPool;
use crate::graphql::schema::Schema;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer, Result};
use juniper::http::GraphQLRequest;
use log::debug;
use std::sync::Arc;
use tera::Tera;

pub mod project;


pub async fn index(tmpl: web::Data<tera::Tera>, pool: DataPgPool) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "kouno");
    Ok(HttpResponse::Ok().content_type("text/html").body(tmpl.render("index.html", &ctx)
      .map_err(|_| error::ErrorInternalServerError("Template error"))?))
}