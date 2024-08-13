use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoomCode {
    #[serde(rename = "roomCode")]
    pub room_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct JoinRoomData {
    #[serde(rename = "roomCode")]
    pub room_code: String,
    pub username: String,
}
