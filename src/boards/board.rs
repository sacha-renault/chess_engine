use super::{CastlingRights, ColorBoard};
use crate::pieces::{static_positions as init, Color, Piece};

/// Represents a chess board with separate bitboards for white and black pieces.
#[derive(Debug, Clone)]
pub struct Board {
    pub white: ColorBoard,
    pub black: ColorBoard,
}

impl Board {
    /// Creates a new `Board` with initial positions for white and black pieces.
    ///
    /// # Returns
    /// A new `Board` instance.
    pub fn new() -> Self {
        Board {
            white: ColorBoard {
                pawn: init::WHITE_PAWNS,
                knight: init::WHITE_KNIGHTS,
                bishop: init::WHITE_BISHOPS,
                rook: init::WHITE_ROOKS,
                queen: init::WHITE_QUEEN,
                king: init::WHITE_KING,
                castling_rights: CastlingRights::new(),
                en_passant: 0,
            },
            black: ColorBoard {
                pawn: init::BLACK_PAWNS,
                knight: init::BLACK_KNIGHTS,
                bishop: init::BLACK_BISHOPS,
                rook: init::BLACK_ROOKS,
                queen: init::BLACK_QUEEN,
                king: init::BLACK_KING,
                castling_rights: CastlingRights::new(),
                en_passant: 0,
            },
        }
    }

    /// Returns a combined bitboard of all pieces on the board.
    ///
    /// # Returns
    /// A `u64` representing the combined bitboard.
    pub fn bitboard(&self) -> u64 {
        self.white.bitboard() | self.black.bitboard()
    }

    /// Returns the bitboard for a specific piece type and color.
    ///
    /// # Arguments
    /// * `piece` - A reference to the `Pieces` type.
    /// * `color` - A reference to the `Color` type.
    ///
    /// # Returns
    /// A `u64` representing the bitboard for the specified piece and color.
    pub fn get_bitboard_by_type(&self, piece: Piece, color: Color) -> u64 {
        match color {
            Color::White => self.white.get_bitboard_by_type(piece),
            Color::Black => self.black.get_bitboard_by_type(piece),
        }
    }

    /// Sets the bitboard for a specific piece type and color.
    ///
    /// # Arguments
    /// * `piece` - A reference to the `Pieces` type.
    /// * `color` - A reference to the `Color` type.
    /// * `new_bitboard` - A `u64` representing the new bitboard.
    pub fn set_bitboard_by_type(&mut self, piece: Piece, color: Color, new_bitboard: u64) {
        match color {
            Color::White => self.white.set_bitboard_by_type(piece, new_bitboard),
            Color::Black => self.black.set_bitboard_by_type(piece, new_bitboard),
        }
    }

    /// Returns a vector of tuples containing the bitboard, piece type, and color for each piece.
    ///
    /// # Returns
    /// A `Vec` of tuples where each tuple contains a `u64` bitboard, a reference to `Pieces`, and a reference to `Color`.
    pub fn individual_pieces(&self) -> Vec<(u64, Piece, Color)> {
        let black_pieces = self
            .black
            .individual_pieces()
            .iter()
            .map(|x| (x.0, x.1, Color::Black))
            .collect::<Vec<_>>();
        let white_pieces = self
            .white
            .individual_pieces()
            .iter()
            .map(|x| (x.0, x.1, Color::White))
            .collect::<Vec<_>>();
        let mut all_pieces = black_pieces;
        all_pieces.extend(white_pieces);
        all_pieces
    }
}
