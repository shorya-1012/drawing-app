use crate::{
    controllers::room_controllers::{create_room_handler, room_exits},
    models::state::AppState,
};
use axum::{
    routing::{get, post},
    Json, Router,
};

pub fn setup_routes(db_pool: AppState) -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/create-room", post(create_room_handler))
        .route("/check-room", post(room_exits))
        .with_state(db_pool)
}

async fn greet() -> Json<&'static str> {
    Json("Hey there")
}
