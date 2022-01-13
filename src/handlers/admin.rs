use crate::db::manager::DataPgPool;
use actix_web::{error, web, Error, HttpResponse, Result};

pub mod project;

pub async fn index(tmpl: web::Data<tera::Tera>, _: DataPgPool) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "hallo world");
    Ok(HttpResponse::Ok().content_type("text/html").body(tmpl.render("index.html", &ctx)
      .map_err(|_| error::ErrorInternalServerError("Template error"))?))
}
