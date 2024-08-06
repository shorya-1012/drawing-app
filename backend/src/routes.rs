use axum::{routing::get, Json, Router};

pub fn setup_routes() -> Router {
    Router::new().route("/", get(greet))
}

async fn greet() -> Json<&'static str> {
    Json("Hey there")
}
