use crate::db::loaders::Loaders;
use crate::db::manager::DataPgPool;
use crate::graphql::schema::{Context, Schema};
use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::env;
use std::sync::Arc;

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    pool: DataPgPool,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut rt = futures::executor::LocalPool::new();
        let ctx = &Context {
            pool: pool.clone(),
            // 中で呼んでるwith_yield_countはどのくらい別スレッドの処理を待機してから実行するかを決める fieldの個数ではないことに注意
            // 内部的にyield_nowを利用しているのでspin-loopしている様子(https://stackoverflow.com/questions/35969884/spin-waits-spin-loop-and-busy-spin)
            // issueとかを読む限りはjuniper側が同レベルのフィールド解決を待たずに先のノードを見に行くせいっぽい
            // 詳細: https://github.com/cksac/dataloader-rs/issues/12
            loaders: Loaders::new(&pool),
        };
        let async_res = data.execute(&st, ctx);
        let res = rt.run_until(async_res);
        // serde_jsonで実際はエラーになる可能性があるのでOkのturbofishに指定
        // 実際これでエラーになる場合はawait?によって早期returnされてErrがreturnされる
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

pub async fn graphiql() -> HttpResponse {
    let endpoint = match env::var("GRAPHQL_ENDPOINT_URL") {
        Ok(val) => val,
        Err(_) => "http://127.0.0.1:3000/graphql".to_string(),
    };

    let html = graphiql_source(&endpoint, None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
