mod board;

use board as board_module;

fn main() {
    let board = board_module::board::Board::new();
    println!("board : {}", board.any_piece_position());
    println!("Queen moves : {}", board_module::moves::queen_moves(board.white.queen, board.white.bitboard(), board.black.bitboard()));
    println!("Knights moves : {}", board_module::moves::knight_moves(board.white.knight, board.white.bitboard()));
    println!("White pawn moves : {}", board_module::moves::pawn_moves(board.white.pawn, board.white.bitboard(), board.black.bitboard(), board::board::Color::White));
    println!("Black pawn moves : {}", board_module::moves::pawn_moves(board.black.pawn, board.black.bitboard(), board.white.bitboard(), board::board::Color::Black));
}
