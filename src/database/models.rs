use serde::{Deserialize, Serialize};

use crate::lichess_api::models::LichessMove;

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

impl MoveModel {
    pub fn to_eval(&self) -> f32 {
        // Draws are weighted at 0.5 for both sides
        let white_score = self.win_rate + (self.draw_rate * 0.5);
        let black_score = self.loose_rate + (self.draw_rate * 0.5);
        
        // Convert to centipawn-like scale (multiply by 100)
        // This makes it more comparable with typical engine evaluations
        ((white_score - black_score) * 100.0) as f32
    }

    pub fn from_lichess_move(mv: &LichessMove, board_id: i64) -> Self {
        let total_games = mv.white + mv.black + mv.draws;
        MoveModel {
            board_id: board_id,
            san: mv.san.clone(),
            win_rate: mv.white as f64 / total_games as f64,
            draw_rate: mv.draws as f64 / total_games as f64,
            loose_rate: mv.black as f64 / total_games as f64,
            game_number: total_games as i64,
            id: None
        }
    }
}