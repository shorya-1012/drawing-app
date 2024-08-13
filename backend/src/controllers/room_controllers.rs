use crate::models::{
    draw_lines_models::{RoomData, UsernameData},
    error_json::ErrorJson,
    room_models::JoinRoomData,
    state::AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Error, Row};

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
    let db_pool = &state.lock().await.db;

    let room_id = generate_room_id();
    let members: Vec<String> = Vec::new();

    let query = "INSERT INTO rooms (room_id , admin, members) VALUES ($1 , $2 , $3)";

    match sqlx::query(query)
        .bind(&room_id)
        .bind(&payload.username)
        .bind(&members)
        .execute(db_pool)
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

    let response = RoomData { room_code: room_id };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn join_room(
    State(state): State<AppState>,
    Json(payload): Json<JoinRoomData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db_pool = &state.lock().await.db;

    let query = "SELECT members from rooms where room_id = $1";

    let record = match sqlx::query(query)
        .bind(&payload.room_code)
        .fetch_one(db_pool)
        .await
    {
        Ok(value) => value,
        Err(Error::RowNotFound) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorJson {
                    error: "Room not found".to_string(),
                }),
            ));
        }
        Err(err) => {
            println!("{:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Database error".to_string(),
                }),
            ));
        }
    };

    let members: Vec<String> = record.get("members");

    // join the room , i.e, add the user to the members list
    if members.contains(&payload.username) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorJson {
                error: "User with username already exists in room".to_string(),
            }),
        ));
    }

    let append_query = "update rooms set members = array_append(members , $1) where room_id = $2";

    match sqlx::query(append_query)
        .bind(&payload.username)
        .bind(&payload.room_code)
        .execute(db_pool)
        .await
    {
        Ok(_) => println!("Joined Room in db"),
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to join room in database".to_string(),
                }),
            ));
        }
    };

    Ok((StatusCode::OK, Json("Room joined successfully")))
}
