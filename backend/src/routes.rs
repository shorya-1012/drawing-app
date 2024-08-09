use axum::{routing::get, Json, Router};

use crate::controllers::room_controllers::create_room_handler;

pub fn setup_routes() -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/create-room", get(create_room_handler))
}

async fn greet() -> Json<&'static str> {
    Json("Hey there")
}
