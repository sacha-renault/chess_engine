use super::static_positions as init;

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
}
