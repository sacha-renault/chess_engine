use crate::game_engine::engine::Engine;
use crate::pieces::{Color, Piece};
use crate::game_engine::player_move::PlayerMove;

use super::super::evaluator_trait::Evaluator;
use super::super::values::*;
use super::utility::classic_heuristic_move_bonus;

pub struct BasicEvaluator {}

impl BasicEvaluator {
    pub fn new() -> Self {
        BasicEvaluator {}
    }

    pub fn get_value_by_piece(piece: Piece, color: Color, bitboard: u64) -> f32 {
        let index = bitboard.trailing_zeros() as usize;
        match piece {
            Piece::Pawn => match color {
                Color::White => WHITE_PAWNS_VALUE[index],
                Color::Black => BLACK_PAWNS_VALUE[index],
            },
            Piece::Bishop => BISHOPS_VALUE[index],
            Piece::Knight => KNIGHTS_VALUE[index],
            _ => 1.25,
        }
    }
}

impl Evaluator for BasicEvaluator {
    fn evaluate_engine_state(&self, engine: &Engine, _: usize) -> f32 {
        let board = engine.get_board();
        let mut score: f32 = 0.;
        for it in board.individual_pieces() {
            let position = it.0;
            let piece = it.1;
            let color = it.2;
            let piece_score =
                get_value_by_piece(piece) * (1. + Self::get_value_by_piece(piece, color, position));
            score += piece_score * ((color as isize) as f32);
        }
        score
    }

    fn evaluate_heuristic_move(
        &self,
        player_move: PlayerMove,
        moved_piece: Piece,
        captured_piece_opt: Option<Piece>,
        is_king_checked: bool
    ) -> f32 {
        classic_heuristic_move_bonus(player_move, moved_piece, captured_piece_opt, is_king_checked)
    }
}