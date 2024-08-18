use crate::{
    models::{
        draw_lines_models::{RoomData, RoomDataResponse, UsernameData},
        error_json::ErrorJson,
        room_models::JoinRoomData,
        state::AppState,
    },
    utils::auth_helpers::get_user_id,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Error, Row};
use tower_cookies::Cookies;

fn generate_room_id() -> String {
    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    id
}

pub async fn create_room_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db_pool = &state.lock().await.db;

    let room_id = generate_room_id();

    let query = "INSERT INTO rooms (roomid) VALUES ($1)";

    match sqlx::query(query).bind(&room_id).execute(db_pool).await {
        Ok(_) => println!("Created"),
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to create object in database".to_string(),
                }),
            ));
        }
    };

    let response = RoomDataResponse { room_code: room_id };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn check_room(
    State(state): State<AppState>,
    Json(payload): Json<JoinRoomData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let state = &state.lock().await;
    let db_pool = &state.db;

    let query = "SELECT COUNT(*) FROM rooms where roomid = $1";

    let exists: (i64,) = match sqlx::query_as(query)
        .bind(&payload.room_code)
        .fetch_one(db_pool)
        .await
    {
        Ok(val) => val,
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to find room".to_string(),
                }),
            ));
        }
    };

    if exists.0 == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorJson {
                error: "Room does not exist".to_string(),
            }),
        ));
    }

    Ok((StatusCode::OK, Json("Room joined successfully")))
}
