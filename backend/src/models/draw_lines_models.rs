use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DrawLine {
    #[serde(rename = "prevPoint")]
    prev_point: Point,
    #[serde(rename = "currPoint")]
    curr_point: Point,
    #[serde(rename = "roomCode")]
    pub room_code: String,
    color: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoomData {
    #[serde(rename = "roomCode")]
    pub room_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientState {
    pub state: String,
    #[serde(rename = "roomCode")]
    pub room_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Point {
    x: f32,
    y: f32,
}
