use axum::{Router, routing::get};

pub async fn app() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
