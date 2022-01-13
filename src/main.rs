use crate::db::manager::new_pool;
use crate::graphql::schema::create_schema;
use crate::router::routes;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, App, HttpServer};
use dotenv::{dotenv, from_filename};
use std::env;
use tera::Tera;

// ログ出したいとき
// #[macro_use]
// extern crate log;

#[macro_use]
extern crate diesel;

extern crate r2d2;
extern crate r2d2_postgres;

pub mod db;
pub mod graphql;
pub mod handlers;
pub mod router;
pub mod schema;
pub mod utils;
pub mod view_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut workers = 4;
    let mut use_unix_socket = true;
    if cfg!(debug_assertions) {
        // debugのときは.env.localファイルを読み込み
        from_filename(".env.local").ok();
        workers = 1;
        use_unix_socket = false;
    } else {
        dotenv().ok();
    }
    env_logger::init();

    let pool = match new_pool() {
        Ok(pool) => pool,
        Err(e) => panic!(e.to_string()),
    };

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());
    let allow_cors = match env::var("ALLOW_CORS") {
        Ok(val) => val,
        Err(_) => "http://localhost".to_string(),
    };

    let mut server = HttpServer::new(move || {

        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(tera)
            .data(pool.clone())
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                // Construct CORS middleware builder
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin(&allow_cors)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .configure(routes)
    })
    .workers(workers);

    server = if use_unix_socket {
        server.bind_uds("/tmp/wbs_api.socket")?
    } else {
        server.bind("0.0.0.0:3000")?
    };
    server.run().await
}
