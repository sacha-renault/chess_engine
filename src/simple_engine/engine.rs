use super::board::Board;
use super::static_positions;

pub struct Engine {
    // rules
    board: Board,
    white_turn: bool,
    en_passant: Option<usize>, // Optional field for en passant target square
    castling_rights: (bool, bool, bool, bool), // (white_king_side, white_queen_side, black_king_side, black_queen_side)
    halfmove_clock: u32, // Number of halfmoves since the last pawn move or capture
    fullmove_number: u32, // Number of full moves in the game
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::new(),
            white_turn: true,
            en_passant: None,
            castling_rights: (true, true, true, true),
            halfmove_clock: 0,
            fullmove_number: 0
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn pawn_moves(&self) -> u64 {
        todo!()
    }


    pub fn rooks_moves(&self) -> u64 {
        todo!()
    }

    pub fn strait_moves(&self) -> u64 {
        todo!()
    }

    pub fn diagonal_moves(&self) -> u64 {
        todo!()
    }

    pub fn queen_moves(&self) -> u64 {
        self.strait_moves() | self.diagonal_moves()
    }

    pub fn king_moves(&self) -> u64 {
        todo!()
    }
}