use serde::Serialize;

#[derive(Serialize)]
pub struct CreateRoom {
    #[serde(rename = "roomCode")]
    pub room_code: String,
}
