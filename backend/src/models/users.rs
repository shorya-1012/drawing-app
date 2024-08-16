use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub userid: String,
    pub password: String,
    pub is_verified: bool,
}

#[derive(Serialize, Deserialize)]
pub struct UserRegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub gender: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSessionIdData {
    pub session_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserIdData {
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerificationData {
    pub verification_code: String,
}
