use axum::{routing::get, Router};

mod fetch;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/fetch_gpt", get(fetch::fetch_gpt::handler_fetch_gpt));
    // localhost:3000 で hyper と共に実行する
    axum::Server::bind(&"0.0.0.0:3015".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
