use super::static_positions as init;
use super::pieces::{Pieces, ALL_PIECES};

#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone)]
pub struct ColorBoard {
    pub pawn: u64,
    pub knight: u64,
    pub bishop: u64,
    pub rook: u64,
    pub queen: u64,
    pub king: u64,
}

impl ColorBoard {
    pub fn bitboard(&self) -> u64 {
        self.pawn | self.knight | self.bishop | self.rook | self.queen | self.king
    }

    pub fn get_bitboard_by_type(&self, piece: &Pieces) -> u64 {
        match piece {
            Pieces::Pawn => self.pawn,
            Pieces::Knight => self.knight,
            Pieces::Bishop => self.bishop,
            Pieces::Rook => self.rook,
            Pieces::Queen => self.queen,
            Pieces::King => self.king,
        }
    }

    pub fn set_bitboard_by_type(&mut self, piece: &Pieces, new_bitboard: u64) {
        match piece {
            Pieces::Pawn => self.pawn = new_bitboard,
            Pieces::Knight => self.knight = new_bitboard,
            Pieces::Bishop => self.bishop = new_bitboard,
            Pieces::Rook => self.rook = new_bitboard,
            Pieces::Queen => self.queen = new_bitboard,
            Pieces::King => self.king = new_bitboard,
        }
    }

    /// Iterate over the pieces one by one and return a Vec
    pub fn individual_pieces(&self) -> Vec<(u64, &Pieces)> {
        let mut result = Vec::new();

        // Iterate over all piece types
        for piece in ALL_PIECES.iter() {
            let bitboard = self.get_bitboard_by_type(&piece);

            // Iterate over the bits of the bitboard
            let mut bitboard_copy = bitboard;
            while bitboard_copy != 0 {
                // Get the position of the least significant bit (LSB)
                let position = bitboard_copy.trailing_zeros() as u64;

                // Add the (position, piece) pair to the result
                result.push((position, piece));

                // Clear the LSB
                bitboard_copy &= bitboard_copy - 1;
            }
        }

        result
    }
}

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
        // Get individual pieces for black and white
        let black_pieces = self.black.individual_pieces()
            .iter().map(|x| (x.0, x.1, &Color::Black)).collect::<Vec<_>>();
        let white_pieces = self.white.individual_pieces()
            .iter().map(|x| (x.0, x.1, &Color::White)).collect::<Vec<_>>();

        // Combine both black and white pieces into a single Vec
        let mut all_pieces = black_pieces;
        all_pieces.extend(white_pieces);
        all_pieces
    }
}
