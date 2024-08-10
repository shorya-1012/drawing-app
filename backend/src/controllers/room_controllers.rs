use crate::models::{
    draw_lines_models::UsernameData, error_json::ErrorJson, room_models::RoomCode, state::AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use rand::{distributions::Alphanumeric, Rng};

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
    Json(payload): Json<UsernameData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db_pool = state.lock().await;

    let members = vec![payload.username];
    let room_id = generate_room_id();

    let query = "INSERT INTO rooms (room_id , members) VALUES ($1 , $2)";

    match sqlx::query(query)
        .bind(&room_id)
        .bind(&members)
        .execute(&*db_pool)
        .await
    {
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

    let response = RoomCode { room_code: room_id };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn room_exits(
    State(state): State<AppState>,
    Json(payload): Json<RoomCode>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db_pool = state.lock().await;

    let query = "SELECT COUNT(*) FROM rooms WHERE room_id = $1";

    let count: (i64,) = match sqlx::query_as(query)
        .bind(payload.room_code)
        .fetch_one(&*db_pool)
        .await
    {
        Ok(value) => value,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if count.0 <= 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok((StatusCode::OK, Json("Room was found")))
}
