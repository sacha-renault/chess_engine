use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveModel {
    pub id: i64,
    pub board_id: i64,
    pub move_text: String,
    pub win_rate: f32,
}