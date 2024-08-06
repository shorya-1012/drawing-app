use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DrawLine {
    #[serde(rename = "prevPoint")]
    prev_point: Point,
    #[serde(rename = "currPoint")]
    curr_point: Point,
    color: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Point {
    x: f32,
    y: f32,
}
