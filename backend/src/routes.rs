use crate::{
    controllers::{
        room_controllers::{check_room, create_room_handler},
        users::{
            get_user_id_handler, login_user_handler, register_user_handler, sign_out_handler,
            verify_user_handler,
        },
    },
    models::state::AppState,
};
use axum::{
    routing::{get, post},
    Json, Router,
};
use tower_cookies::CookieManagerLayer;

pub fn setup_routes(db_pool: AppState) -> Router {
    Router::new()
        .route("/", get(greet))
        .route("/register", post(register_user_handler))
        .route("/login", post(login_user_handler))
        .route("/verify", post(verify_user_handler))
        .route("/sign-out", post(sign_out_handler))
        .route("/get-user-id", get(get_user_id_handler))
        .route("/create-room", post(create_room_handler))
        .route("/check-room", post(check_room))
        .layer(CookieManagerLayer::new())
        .with_state(db_pool)
}

async fn greet() -> Json<&'static str> {
    Json("Hey there")
}
