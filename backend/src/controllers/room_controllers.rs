use crate::models::room_models::CreateRoom;
use axum::{http::StatusCode, response::IntoResponse, Json};

use rand::{distributions::Alphanumeric, Rng};

fn generate_room_id() -> String {
    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    id
}

pub async fn create_room_handler() -> impl IntoResponse {
    let response = CreateRoom {
        room_code: generate_room_id(),
    };

    (StatusCode::CREATED, Json(response))
}
