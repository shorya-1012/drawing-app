use serde::{Deserialize, Serialize};

#[derive(Serialize , Deserialize)]
pub struct RoomCode {
    #[serde(rename = "roomCode")]
    pub room_code: String,
}
