use super::color::Color;
use super::color_board::ColorBoard;
use super::pieces::Pieces;
use super::static_positions as init;

#[derive(Clone)]
pub struct Board {
    pub white: ColorBoard,
    pub black: ColorBoard,
}

impl Board {
    pub fn new() -> Self {
        Board {
            white: ColorBoard {
                pawn: init::WHITE_PAWNS,
                knight: init::WHITE_KNIGHTS,
                bishop: init::WHITE_BISHOPS,
                rook: init::WHITE_ROOKS,
                queen: init::WHITE_QUEEN,
                king: init::WHITE_KING,
            },
            black: ColorBoard {
                pawn: init::BLACK_PAWNS,
                knight: init::BLACK_KNIGHTS,
                bishop: init::BLACK_BISHOPS,
                rook: init::BLACK_ROOKS,
                queen: init::BLACK_QUEEN,
                king: init::BLACK_KING,
            },
        }
    }

    pub fn bitboard(&self) -> u64 {
        self.white.bitboard() | self.black.bitboard()
    }

    pub fn get_bitboard_by_type(&self, piece: &Pieces, color: &Color) -> u64 {
        match color {
            Color::White => self.white.get_bitboard_by_type(&piece),
            Color::Black => self.black.get_bitboard_by_type(&piece),
        }
    }

    pub fn set_bitboard_by_type(&mut self, piece: &Pieces, color: &Color, new_bitboard: u64) {
        match color {
            Color::White => self.white.set_bitboard_by_type(&piece, new_bitboard),
            Color::Black => self.black.set_bitboard_by_type(&piece, new_bitboard),
        }
    }

    pub fn individual_pieces(&self) -> Vec<(u64, &Pieces, &Color)> {
        let black_pieces = self
            .black
            .individual_pieces()
            .iter()
            .map(|x| (x.0, x.1, &Color::Black))
            .collect::<Vec<_>>();
        let white_pieces = self
            .white
            .individual_pieces()
            .iter()
            .map(|x| (x.0, x.1, &Color::White))
            .collect::<Vec<_>>();
        let mut all_pieces = black_pieces;
        all_pieces.extend(white_pieces);
        all_pieces
    }
}
