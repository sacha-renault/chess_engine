use super::{Board, ColorBoard};
use crate::pieces::Color;
use rand::Rng;
use once_cell::sync::Lazy;

const NUM_SQUARES: usize = 64;
const NUM_PIECE_TYPES: usize = 6; // Pawn, Knight, Bishop, Rook, Queen, King
const NUM_COLORS: usize = 2; // White, Black

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Zobrist {
    table: [[[u64; NUM_SQUARES]; NUM_COLORS]; NUM_PIECE_TYPES],
    castling_rights: [u64; 16], // 16 possible combinations of castling rights
    en_passant: [u64; NUM_SQUARES],
    side_to_move: u64,
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            table: [[[0; NUM_SQUARES]; NUM_COLORS]; NUM_PIECE_TYPES]
                .map(|c| c.map(|s| s.map(|_| rng.gen::<u64>()))),
            castling_rights: [0; 16].map(|_| rng.gen::<u64>()),
            en_passant: [0; NUM_SQUARES].map(|_| rng.gen::<u64>()),
            side_to_move: rng.gen::<u64>(),
        }
    }

    pub fn compute_hash(&self, board: &Board, white_to_play: bool) -> u64 {
        let mut hash: u64 = 0;

        // Hash all white pieces
        hash ^= self.hash_color_board(&board.white, Color::White);

        // Hash all black pieces
        hash ^= self.hash_color_board(&board.black, Color::Black);

        // Add castling rights
        let castling_index =
            board.white.castling_rights.as_index() | (board.black.castling_rights.as_index() << 2);
        hash ^= self.castling_rights[castling_index];

        // Add en passant
        if board.white.en_passant != 0 {
            hash ^= self.en_passant[board.white.en_passant.trailing_zeros() as usize];
        }
        if board.black.en_passant != 0 {
            hash ^= self.en_passant[board.black.en_passant.trailing_zeros() as usize];
        }

        // Add side to move
        if !white_to_play {
            hash ^= self.side_to_move;
        }

        hash
    }

    fn hash_color_board(&self, color_board: &ColorBoard, color: Color) -> u64 {
        let mut hash: u64 = 0;
        let color_index = {
            if color == Color::White {
                0
            } else {
                1
            }
        };

        // Hash each piece type
        for (piece_type, bitboard) in [
            (PieceType::Pawn, color_board.pawn),
            (PieceType::Knight, color_board.knight),
            (PieceType::Bishop, color_board.bishop),
            (PieceType::Rook, color_board.rook),
            (PieceType::Queen, color_board.queen),
            (PieceType::King, color_board.king),
        ] {
            let mut bb = bitboard;
            while bb != 0 {
                let square = bb.trailing_zeros() as usize;
                hash ^= self.table[piece_type as usize][color_index][square];
                bb &= bb - 1; // Clear the least significant bit
            }
        }

        hash
    }
}

pub static HASHER: Lazy<Zobrist> = Lazy::new(|| Zobrist::new());
