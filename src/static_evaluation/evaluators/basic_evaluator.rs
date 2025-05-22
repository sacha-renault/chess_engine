use crate::game_engine::engine::Engine;
use crate::game_engine::player_move::PlayerMove;
use crate::pieces::Piece;

use super::super::evaluator_trait::Evaluator;
use super::utility::{
    classic_heuristic_move_bonus, get_value_by_piece, get_value_multiplier_by_piece,
};

pub struct BasicEvaluator {}

impl BasicEvaluator {
    pub fn new() -> Self {
        BasicEvaluator {}
    }
}

impl Evaluator for BasicEvaluator {
    fn evaluate_engine_state(&self, engine: &Engine, _: usize) -> f32 {
        let board = engine.get_board();
        let mut score: f32 = 0.;
        let neg = if engine.white_to_play() { 1. } else { -1. };
        for it in board.individual_pieces() {
            let position = it.0;
            let piece = it.1;
            let color = it.2;
            let piece_score = get_value_by_piece(piece)
                * (1. + get_value_multiplier_by_piece(piece, color, position));
            score += piece_score * ((color as isize) as f32);
        }
        score * neg
    }

    fn evaluate_heuristic_move(
        &self,
        player_move: PlayerMove,
        moved_piece: Piece,
        captured_piece_opt: Option<Piece>,
        is_king_checked: bool,
    ) -> f32 {
        classic_heuristic_move_bonus(
            player_move,
            moved_piece,
            captured_piece_opt,
            is_king_checked,
        )
    }
}
