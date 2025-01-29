use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardModel {
    pub id: Option<i64>,
    pub fen: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveModel {
    pub id: Option<i64>,
    pub board_id: i64,
    pub san: String,
    pub win_rate: f64,
    pub draw_rate: f64,
    pub loose_rate: f64,
    pub game_number: i64,
}