use super::pieces::{Pieces, ALL_PIECES};

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

    pub fn individual_pieces(&self) -> Vec<(u64, &Pieces)> {
        let mut result = Vec::new();
        for piece in ALL_PIECES.iter() {
            let bitboard = self.get_bitboard_by_type(&piece);
            let mut bitboard_copy = bitboard;
            while bitboard_copy != 0 {
                let position = bitboard_copy.trailing_zeros() as u64;
                result.push((position, piece));
                bitboard_copy &= bitboard_copy - 1;
            }
        }
        result
    }
}