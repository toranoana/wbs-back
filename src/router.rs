use crate::handlers::api::v1::project::{task, user};
use crate::handlers::graphql::*;
use crate::handlers::home::index;
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(index))
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        .service(
            web::scope("/project/{project_id}")
                .service(
                    web::scope("/user")
                        .service(web::resource("/import").route(web::post().to(user::import))),
                )
                .service(
                    web::scope("/task")
                        .service(web::resource("/import").route(web::post().to(task::import))),
                ),
        );
}
