use actix_web::{HttpRequest, Responder};

pub async fn index(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}
