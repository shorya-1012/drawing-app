use crate::{
    models::{
        error_json::ErrorJson,
        state::AppState,
        users::{UserData, UserIdData, UserLoginData, UserRegisterData, VerificationData},
    },
    utils::{
        emails::send_email,
        helpers::{generate_session_id, generate_uuid},
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use redis::Commands;
use sqlx::{Error, Row};
use tower_cookies::{Cookie, Cookies};

pub async fn register_user_handler(
    State(state): State<AppState>,
    Json(user_data): Json<UserRegisterData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db = &state.lock().await.db;
    let user_id = generate_uuid();

    let hash_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to hash password".to_string(),
                }),
            ));
        }
    };

    let query =
            "INSERT INTO app_users (userid , username, password ,email, gender) VALUES ($1 , $2 , $3 , $4 , $5)";

    match sqlx::query(query)
        .bind(&user_id)
        .bind(&user_data.username)
        .bind(hash_password)
        .bind(&user_data.email)
        .bind(&user_data.gender)
        .execute(db)
        .await
    {
        Ok(_) => println!("Created"),
        Err(Error::Database(_)) => {
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorJson {
                    error: "User with username already exists".to_string(),
                }),
            ));
        }
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

    let _ = send_email(db, user_id, user_data.username, user_data.email).await;

    Ok((StatusCode::OK, Json("User registered successfully")))
}

pub async fn login_user_handler(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(user_data): Json<UserLoginData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let app_state = &state.lock().await;
    let db = &app_state.db;
    let redis_client = &app_state.redis;

    let user_search_query =
        "SELECT userid , password , is_verified FROM app_users WHERE username= $1 OR email = $1";
    let user_record = match sqlx::query(user_search_query)
        .bind(&user_data.username)
        .fetch_one(db)
        .await
    {
        Ok(value) => value,
        Err(Error::RowNotFound) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorJson {
                    error: "User does not exist".to_string(),
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

    let user = UserData {
        userid: user_record.get("userid"),
        password: user_record.get("password"),
        is_verified: user_record.get("is_verified"),
    };

    if !user.is_verified {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorJson {
                error: "User is not verified. Cannot login".to_string(),
            }),
        ));
    }

    let is_valid_password = match verify(&user_data.password, &user.password) {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Error verifying password".to_string(),
                }),
            ));
        }
    };

    if !is_valid_password {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorJson {
                error: "Incorrect Password provided".to_string(),
            }),
        ));
    }

    let mut con = match redis_client.get_connection() {
        Ok(client) => client,
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to form connection with redis client".to_string(),
                }),
            ));
        }
    };

    let session_id = generate_session_id();

    let mut cookie = Cookie::new("session_id", session_id.clone());
    cookie.set_same_site(None);
    cookie.set_secure(false);
    cookies.add(cookie);

    let _: () = match con.set(&session_id, user.userid) {
        Ok(res) => res,
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to set session id in redis databse".to_string(),
                }),
            ));
        }
    };

    Ok((StatusCode::OK, Json("User LogIn successful")))
}

pub async fn get_user_id_handler(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let redis_client = &state.lock().await.redis;

    let session_id = match cookies.get("session_id") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorJson {
                    error: "Sessions Id cookie not found".to_string(),
                }),
            ));
        }
    };

    let mut con = match redis_client.get_connection() {
        Ok(client) => client,
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to form connection with redis client".to_string(),
                }),
            ));
        }
    };

    let user_id: String = match con.get(session_id) {
        Ok(client) => client,
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to get user session id".to_string(),
                }),
            ));
        }
    };

    Ok((StatusCode::OK, Json(UserIdData { user_id })))
}

pub async fn verify_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<VerificationData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db = &state.lock().await.db;

    let query = "SELECT userid from verification_codes WHERE code = $1 AND expiration_time > now()";

    let record = match sqlx::query(query)
        .bind(&payload.verification_code)
        .fetch_one(db)
        .await
    {
        Ok(value) => value,
        Err(Error::RowNotFound) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorJson {
                    error: "Verification Code Expired or Not found".to_string(),
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

    let user_id: String = record.get("userid");

    let update_user_query = "UPDATE app_users SET is_verified = TRUE WHERE userid = $1";
    match sqlx::query(update_user_query)
        .bind(&user_id)
        .execute(db)
        .await
    {
        Ok(_) => println!("User verified"),
        Err(err) => {
            println!("{:#?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorJson {
                    error: "Unable to update user status".to_string(),
                }),
            ));
        }
    };

    Ok((StatusCode::OK, Json("User has been verified")))
}

// pub async fn get_verification_email(
//     State(state): State<AppState>,
//     Json(payload): Json<VerificationData>,
// ) -> Result<impl IntoResponse, impl IntoResponse> {
//     // let db = &state.lock().await.db;

//     // Ok((StatusCode::OK, Json("User has been verified")))
// }
