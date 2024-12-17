mod simple_engine;

use simple_engine::board::{Board, Color};
use simple_engine::moves;

fn main() {
    let board = Board::new();
    println!("board : {}", board.any_piece_position());
    println!("Queen moves : {}", moves::queen_moves(board.white.queen, board.white.bitboard(), board.black.bitboard()));
    println!("Knights moves : {}", moves::knight_moves(board.white.knight, board.white.bitboard()));
    println!("White pawn moves : {}", moves::pawn_moves(board.white.pawn, board.white.bitboard(), board.black.bitboard(), Color::White));
    println!("Black pawn moves : {}", moves::pawn_moves(board.black.pawn, board.black.bitboard(), board.white.bitboard(), Color::Black));
}
