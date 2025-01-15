use super::evaluate::Evaluator;
use crate::boards::board::Board;
use crate::pieces::{Color, Piece};

pub const CHECK_MATE_VALUE: f32 = 1e5 as f32;
pub const WHITE_PAWNS_VALUE: [f32; 64] = [
    0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
    0.82, 0.85, 0.92, 1.25, 1.25, 0.92, 0.85, 0.82,
    0.94, 0.97, 1.04, 1.42, 1.42, 1.04, 0.97, 0.94,
    1.06, 1.1,  1.18, 1.61, 1.61, 1.18, 1.1,  1.06,
    1.17, 1.22, 1.31, 1.79, 1.79, 1.31, 1.22, 1.17,
    1.29, 1.33, 1.44, 1.96, 1.96, 1.44, 1.33, 1.29,
    1.4,  1.45, 1.57, 2.14, 2.14, 1.57, 1.45, 1.4,
    1.53, 1.58, 1.7,  2.33, 2.33, 1.7,  1.58, 1.53
];
pub const BLACK_PAWNS_VALUE: [f32; 64] = [
    1.53, 1.58, 1.7,  2.33, 2.33, 1.7,  1.58, 1.53,
    1.4,  1.45, 1.57, 2.14, 2.14, 1.57, 1.45, 1.4,
    1.29, 1.33, 1.44, 1.96, 1.96, 1.44, 1.33, 1.29,
    1.17, 1.22, 1.31, 1.79, 1.79, 1.31, 1.22, 1.17,
    1.06, 1.1,  1.18, 1.61, 1.61, 1.18, 1.1,  1.06,
    0.94, 0.97, 1.04, 1.42, 1.42, 1.04, 0.97, 0.94,
    0.82, 0.85, 0.92, 1.25, 1.25, 0.92, 0.85, 0.82,
    0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
];
pub const BISHOPS_VALUE: [f32; 64] = [
    1.0,  1.0,  0.75, 0.75, 0.75, 0.75, 1.0,  1.0,
    1.0,  1.5,  1.25, 1.0,  1.0,  1.25, 1.5,  1.0,
    0.75, 1.25, 1.5,  1.25, 1.25, 1.5,  1.25, 0.75,
    0.75, 1.0,  1.25, 1.75, 1.75, 1.25, 1.0,  0.75,
    0.75, 1.0,  1.25, 1.75, 1.75, 1.25, 1.0,  0.75,
    0.75, 1.25, 1.5,  1.25, 1.25, 1.5,  1.25, 0.75,
    1.0,  1.5,  1.25, 1.0,  1.0,  1.25, 1.5,  1.0,
    1.0,  1.0,  0.75, 0.75, 0.75, 0.75, 1.0,  1.0,
];
pub const KNIGHTS_VALUE: [f32; 64] = [
    0.7,  0.73, 0.76, 0.78, 0.78, 0.76, 0.73, 0.7,
    0.73, 0.78, 0.84, 0.89, 0.89, 0.84, 0.78, 0.73,
    0.76, 0.84, 0.97, 1.13, 1.13, 0.97, 0.84, 0.76,
    0.78, 0.89, 1.13, 1.91, 1.91, 1.13, 0.89, 0.78,
    0.78, 0.89, 1.13, 1.91, 1.91, 1.13, 0.89, 0.78,
    0.76, 0.84, 0.97, 1.13, 1.13, 0.97, 0.84, 0.76,
    0.73, 0.78, 0.84, 0.89, 0.89, 0.84, 0.78, 0.73,
    0.7,  0.73, 0.76, 0.78, 0.78, 0.76, 0.73, 0.7,
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

pub struct ValueRuleSet { }

impl ValueRuleSet {
    pub fn new() -> Self {
        ValueRuleSet { }
    }

    pub fn get_table(piece: Piece, color: Color) -> [f32; 64] {
        match piece {
            Piece::Pawn => {
                match color {
                    Color::White => WHITE_PAWNS_VALUE,
                    Color::Black => BLACK_PAWNS_VALUE,
                }
            },
            Piece::Bishop => BISHOPS_VALUE,
            Piece::Knight => KNIGHTS_VALUE,
            _ => [1.25; 64],
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
                get_value_by_piece(piece) * (1. + Self::get_table(piece, color)[position.trailing_ones() as usize]);
            score += piece_score * ((color as isize) as f32);
        }
        score
    }
}
