use super::castling_rights::CastlingRights;
use crate::pieces::Piece;

/// Represents a color-specific board with bitboards for each piece type.
#[derive(Debug, Clone)]
pub struct ColorBoard {
    pub pawn: u64,
    pub knight: u64,
    pub bishop: u64,
    pub rook: u64,
    pub queen: u64,
    pub king: u64,
    pub castling_rights: CastlingRights,
    pub en_passant: u64,
}

impl ColorBoard {
    /// Returns a combined bitboard of all pieces for the color.
    ///
    /// # Returns
    /// A `u64` representing the combined bitboard.
    pub fn bitboard(&self) -> u64 {
        self.pawn | self.knight | self.bishop | self.rook | self.queen | self.king
    }

    /// Returns the bitboard for a specific piece type.
    ///
    /// # Arguments
    /// * `piece` - A reference to the `Pieces` type.
    ///
    /// # Returns
    /// A `u64` representing the bitboard for the specified piece.
    pub fn get_bitboard_by_type(&self, piece: Piece) -> u64 {
        match piece {
            Piece::Pawn => self.pawn,
            Piece::Knight => self.knight,
            Piece::Bishop => self.bishop,
            Piece::Rook => self.rook,
            Piece::Queen => self.queen,
            Piece::King => self.king,
        }
    }

    /// Sets the bitboard for a specific piece type.
    ///
    /// # Arguments
    /// * `piece` - A reference to the `Pieces` type.
    /// * `new_bitboard` - A `u64` representing the new bitboard.
    pub fn set_bitboard_by_type(&mut self, piece: Piece, new_bitboard: u64) {
        match piece {
            Piece::Pawn => self.pawn = new_bitboard,
            Piece::Knight => self.knight = new_bitboard,
            Piece::Bishop => self.bishop = new_bitboard,
            Piece::Rook => self.rook = new_bitboard,
            Piece::Queen => self.queen = new_bitboard,
            Piece::King => self.king = new_bitboard,
        }
    }

    /// Returns a vector of tuples containing the bitboard position and piece type for each piece.
    ///
    /// # Returns
    /// A `Vec` of tuples where each tuple contains a `u64` bitboard position and a reference to `Pieces`.
    pub fn individual_pieces(&self) -> Vec<(u64, Piece)> {
        let piece_bitboards: [(u64, Piece); 6] = [
            (self.pawn, Piece::Pawn),
            (self.knight, Piece::Knight),
            (self.bishop, Piece::Bishop),
            (self.rook, Piece::Rook),
            (self.queen, Piece::Queen),
            (self.king, Piece::King),
        ];

        let mut result = Vec::new();
        for (bitboard, piece) in piece_bitboards.iter() {
            let mut bitboard_copy = *bitboard;
            while bitboard_copy != 0 {
                let mask = bitboard_copy & (!bitboard_copy + 1);
                result.push((mask, *piece));
                bitboard_copy &= bitboard_copy - 1;
            }
        }
        result
    }
}
