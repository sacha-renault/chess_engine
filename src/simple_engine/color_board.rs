use super::pieces::{Pieces, ALL_PIECES};

// it checks if the castling is authorized
#[derive(Clone, Copy)]
pub struct CastlingRights {
    short: bool,
    long: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            short: true,
            long: true,
        }
    }

    /// Updates the castling rights based on the current state of the king and rook bitboards.
    ///
    /// # Arguments
    /// * `king_bitboard` - The bitboard representing the king's position.
    /// * `rook_bitboard` - The bitboard representing the rooks' positions.
    /// * `initial_king_pos` - The initial position of the king (e.g., `0b1000` for white).
    /// * `initial_short_rook_pos` - The initial position of the short-side rook (e.g., `0b1` for white).
    /// * `initial_long_rook_pos` - The initial position of the long-side rook (e.g., `0b100000000` for white).
    pub fn update_castling_rights(
        &mut self,
        king_bitboard: u64,
        rook_bitboard: u64,
        initial_king_pos: u64,
        initial_short_rook_pos: u64,
        initial_long_rook_pos: u64,
    ) {
        // Disable short castling if the king or short-side rook has moved
        if self.short {
            if king_bitboard & initial_king_pos == 0 || rook_bitboard & initial_short_rook_pos == 0
            {
                self.short = false;
            }
        }

        // Disable long castling if the king or long-side rook has moved
        if self.long {
            if king_bitboard & initial_king_pos == 0 || rook_bitboard & initial_long_rook_pos == 0 {
                self.long = false;
            }
        }
    }

    pub fn long_casting_available(&self, full_bitboard: u64, required_empty: u64) -> bool {
        self.long && (full_bitboard & required_empty == 0)
    }

    pub fn short_casting_available(&self, full_bitboard: u64, required_empty: u64) -> bool {
        self.short && (full_bitboard & required_empty == 0)
    }
}

/// Represents a color-specific board with bitboards for each piece type.
#[derive(Clone)]
pub struct ColorBoard {
    pub pawn: u64,
    pub knight: u64,
    pub bishop: u64,
    pub rook: u64,
    pub queen: u64,
    pub king: u64,
    pub castling_rights: CastlingRights,
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
    pub fn get_bitboard_by_type(&self, piece: Pieces) -> u64 {
        match piece {
            Pieces::Pawn => self.pawn,
            Pieces::Knight => self.knight,
            Pieces::Bishop => self.bishop,
            Pieces::Rook => self.rook,
            Pieces::Queen => self.queen,
            Pieces::King => self.king,
        }
    }

    /// Sets the bitboard for a specific piece type.
    ///
    /// # Arguments
    /// * `piece` - A reference to the `Pieces` type.
    /// * `new_bitboard` - A `u64` representing the new bitboard.
    pub fn set_bitboard_by_type(&mut self, piece: Pieces, new_bitboard: u64) {
        match piece {
            Pieces::Pawn => self.pawn = new_bitboard,
            Pieces::Knight => self.knight = new_bitboard,
            Pieces::Bishop => self.bishop = new_bitboard,
            Pieces::Rook => self.rook = new_bitboard,
            Pieces::Queen => self.queen = new_bitboard,
            Pieces::King => self.king = new_bitboard,
        }
    }

    /// Returns a vector of tuples containing the bitboard position and piece type for each piece.
    ///
    /// # Returns
    /// A `Vec` of tuples where each tuple contains a `u64` bitboard position and a reference to `Pieces`.
    pub fn individual_pieces(&self) -> Vec<(u64, Pieces)> {
        let piece_bitboards: [(u64, Pieces); 6] = [
            (self.pawn, Pieces::Pawn),
            (self.knight, Pieces::Knight),
            (self.bishop, Pieces::Bishop),
            (self.rook, Pieces::Rook),
            (self.queen, Pieces::Queen),
            (self.king, Pieces::King),
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
