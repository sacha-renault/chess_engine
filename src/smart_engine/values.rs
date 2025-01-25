use super::evaluate::Evaluator;
use crate::boards::board::Board;
use crate::pieces::{Color, Piece};

// Evaluator const values
pub const CASTLING_BONUS: f32 = 5.;
pub const CAPTURE_BONUS: f32 = 1.;
pub const CAPTURE_MVV_LVA_FACTOR: f32 = 1.;
pub const CHECK_BONUS: f32 = 5.;
pub const CHECK_MATE: f32 = 1e5 as f32;
pub const WHITE_PAWNS_VALUE: [f32; 64] = [
    0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.87, 0.87, 0.87, 0.9, 0.9, 0.87, 0.87, 0.87,
    0.9, 0.91, 0.92, 0.98, 0.98, 0.92, 0.91, 0.9, 0.94, 0.95, 0.96, 1.05, 1.05, 0.96, 0.95, 0.94,
    0.98, 0.99, 1.01, 1.12, 1.12, 1.01, 0.99, 0.98, 1.01, 1.03, 1.05, 1.2, 1.2, 1.05, 1.03, 1.01,
    1.05, 1.07, 1.1, 1.27, 1.27, 1.1, 1.07, 1.05, 1.09, 1.1, 1.14, 1.35, 1.35, 1.14, 1.1, 1.09,
];
pub const BLACK_PAWNS_VALUE: [f32; 64] = [
    1.09, 1.1, 1.14, 1.35, 1.35, 1.14, 1.1, 1.09, 1.05, 1.07, 1.1, 1.27, 1.27, 1.1, 1.07, 1.05,
    1.01, 1.03, 1.05, 1.2, 1.2, 1.05, 1.03, 1.01, 0.98, 0.99, 1.01, 1.12, 1.12, 1.01, 0.99, 0.98,
    0.94, 0.95, 0.96, 1.05, 1.05, 0.96, 0.95, 0.94, 0.9, 0.91, 0.92, 0.98, 0.98, 0.92, 0.91, 0.9,
    0.87, 0.87, 0.87, 0.9, 0.9, 0.87, 0.87, 0.87, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83, 0.83,
];
pub const BISHOPS_VALUE: [f32; 64] = [
    0.95, 0.95, 0.83, 0.83, 0.83, 0.83, 0.95, 0.95, 0.95, 1.19, 1.07, 0.95, 0.95, 1.07, 1.19, 0.95,
    0.83, 1.07, 1.19, 1.07, 1.07, 1.19, 1.07, 0.83, 0.83, 0.95, 1.07, 1.3, 1.3, 1.07, 0.95, 0.83,
    0.83, 0.95, 1.07, 1.3, 1.3, 1.07, 0.95, 0.83, 0.83, 1.07, 1.19, 1.07, 1.07, 1.19, 1.07, 0.83,
    0.95, 1.19, 1.07, 0.95, 0.95, 1.07, 1.19, 0.95, 0.95, 0.95, 0.83, 0.83, 0.83, 0.83, 0.95, 0.95,
];
pub const KNIGHTS_VALUE: [f32; 64] = [
    0.93, 0.95, 0.96, 0.97, 0.97, 0.96, 0.95, 0.93, 0.95, 0.97, 0.99, 1.01, 1.01, 0.99, 0.97, 0.95,
    0.96, 0.99, 1.03, 1.07, 1.07, 1.03, 0.99, 0.96, 0.97, 1.01, 1.07, 1.19, 1.19, 1.07, 1.01, 0.97,
    0.97, 1.01, 1.07, 1.19, 1.19, 1.07, 1.01, 0.97, 0.96, 0.99, 1.03, 1.07, 1.07, 1.03, 0.99, 0.96,
    0.95, 0.97, 0.99, 1.01, 1.01, 0.99, 0.97, 0.95, 0.93, 0.95, 0.96, 0.97, 0.97, 0.96, 0.95, 0.93,
];

pub fn get_value_by_piece(piece: Piece) -> f32 {
    match piece {
        Piece::Pawn => 10.,
        Piece::Bishop => 30.,
        Piece::Knight => 30.,
        Piece::Rook => 50.,
        Piece::Queen => 90.,
        Piece::King => 1e5 as f32,
    }
}

// Tree const values
pub const FORESEEING_WINDOW: f32 = 10.;

pub struct ValueRuleSet {}

impl ValueRuleSet {
    pub fn new() -> Self {
        ValueRuleSet {}
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

impl Evaluator for ValueRuleSet {
    fn evaluate(&self, board: &Board) -> f32 {
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
}
