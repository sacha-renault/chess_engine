// it checks if the castling is authorized
#[derive(Debug, Clone, Copy)]
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

    pub fn new_with_rules(short: bool, long: bool) -> Self {
        CastlingRights { short, long }
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

    pub fn is_long_castling_possible(&self, full_bitboard: u64, required_empty: u64) -> bool {
        self.long && (full_bitboard & required_empty == 0)
    }

    pub fn is_short_castling_possible(&self, full_bitboard: u64, required_empty: u64) -> bool {
        self.short && (full_bitboard & required_empty == 0)
    }

    pub fn is_long_castling_available(&self) -> bool {
        self.long
    }

    pub fn is_short_castling_available(&self) -> bool {
        self.short
    }

    pub fn as_index(&self) -> usize {
        (self.short as usize) | ((self.long as usize) << 1)
    }
}
