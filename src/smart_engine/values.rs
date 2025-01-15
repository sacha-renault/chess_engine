use super::evaluate::Evaluator;
use crate::boards::board::Board;
use crate::pieces::{Color, Piece};

pub const CHECK_MATE_VALUE: f32 = 1e5 as f32;

const fn init_bishop_table() -> [f32; 64] {
    let mut values: [f32; 64] = [0.; 64];
    let mut i = 0;
    while i < 64 {
        let file = i % 8;
        let rank = i / 8;

        // Center control bonus
        let center_bonus = if (file == 3 || file == 4) && (rank == 3 || rank == 4) {
            0.2
        } else {
            0.0
        };

        // Main diagonal bonus
        let diag_bonus = if (file == rank) || (file + rank == 7) {
            0.15
        } else {
            0.0
        };

        // Edge penalty
        let edge_penalty = if file == 0 || file == 7 || rank == 0 || rank == 7 {
            -0.2
        } else {
            0.0
        };

        values[i] = center_bonus + diag_bonus + edge_penalty;
        i += 1;
    }
    values
}

const fn init_pawns(start: f32, end: f32) -> [f32; 64] {
    let mut i = 0;
    let mut values: [f32; 64] = [0.; 64];
    while i < 8 {
        let mut j = 0;
        let mul = (i + 1) as f32;
        while j < 8 {
            values[i * 8 + j] = start * mul / 8. + end * (8. - mul) / 8.;
            j += 1;
        }
        i += 1;
    }
    values
}

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

const WHITE_PAWN_TABLE: [f32; 64] = init_pawns(0., 1.);
const BLACK_PAWN_TABLE: [f32; 64] = init_pawns(1., 0.);
const BISHOP_TABLE: [f32; 64] = init_bishop_table();

pub struct ValueRuleSet {
    white_pawn_table: [f32; 64],
    black_pawn_table: [f32; 64],
    knight_table: [f32; 64],
    bishop_table: [f32; 64],
    rook_table: [f32; 64],
    queen_table: [f32; 64],
    king_table: [f32; 64],
}

impl ValueRuleSet {
    pub fn new() -> Self {
        todo!();
    }

    pub fn get_table(piece: Piece, color: Color) -> [f32; 64] {
        todo!();
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
                get_value_by_piece(piece) * (1. + Self::get_table(piece, color)[position as usize]);
            score += piece_score * ((color as isize) as f32);
        }
        score
    }
}
