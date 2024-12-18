use super::board::{Board, Color};
use super::moves::*;
use super::{board::ColorBoard, pieces::Pieces};

pub fn get_piece_type(color_board: &ColorBoard, square: u64) -> Option<Pieces> {
    if color_board.pawn & square != 0 {
        return Some(Pieces::Pawn);
    }

    if color_board.knight & square != 0 {
        return Some(Pieces::Knight);
    }

    if color_board.bishop & square != 0 {
        return Some(Pieces::Bishop);
    }

    if color_board.rook & square != 0 {
        return Some(Pieces::Rook);
    }

    if color_board.queen & square != 0 {
        return Some(Pieces::Queen);
    }

    if color_board.king & square != 0 {
        return Some(Pieces::King);
    }
    None
}

pub fn coordinates_to_u64(position: (usize, usize)) -> u64 {
    let (row, col) = position;

    assert!(
        row < 8 && col < 8,
        "Row and column must be between 0 and 7."
    );

    // Calculate the 0-63 square index
    let square_index = row * 8 + col;

    // Set the bit at the square index
    1u64 << square_index
}

pub fn get_possible_move(
    piece: &Pieces,
    start_square: u64,
    same_color_bitboard: u64,
    other_color_bitboard: u64,
    color: &Color,
) -> u64 {
    let bitboard = start_square & same_color_bitboard;
    match piece {
        Pieces::King => king_moves(bitboard, same_color_bitboard),
        Pieces::Queen => queen_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Pieces::Rook => rooks_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Pieces::Knight => knight_moves(bitboard, same_color_bitboard),
        Pieces::Bishop => bishops_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Pieces::Pawn => pawn_moves(bitboard, same_color_bitboard, other_color_bitboard, &color),
    }
}

pub fn get_color(white_turn: bool) -> Color {
    if white_turn {
        Color::White
    } else {
        Color::Black
    }
}

pub fn move_piece(
    mut board: Board,
    current_square: u64,
    target_square: u64,
    color: &Color,
    piece_type: &Pieces,
) -> Board {
    // Determine which color's board is being modified
    let (color_board, opponent_board) = match color {
        Color::White => (&mut board.white, &mut board.black),
        Color::Black => (&mut board.black, &mut board.white),
    };

    // Clear the current square from the moving piece
    match piece_type {
        Pieces::Pawn => color_board.pawn &= !current_square,
        Pieces::Knight => color_board.knight &= !current_square,
        Pieces::Bishop => color_board.bishop &= !current_square,
        Pieces::Rook => color_board.rook &= !current_square,
        Pieces::Queen => color_board.queen &= !current_square,
        Pieces::King => color_board.king &= !current_square,
    };

    // Place the piece on the target square
    match piece_type {
        Pieces::Pawn => color_board.pawn |= target_square,
        Pieces::Knight => color_board.knight |= target_square,
        Pieces::Bishop => color_board.bishop |= target_square,
        Pieces::Rook => color_board.rook |= target_square,
        Pieces::Queen => color_board.queen |= target_square,
        Pieces::King => color_board.king |= target_square,
    };

    // Clear the target square from all opponent pieces
    opponent_board.pawn &= !target_square;
    opponent_board.knight &= !target_square;
    opponent_board.bishop &= !target_square;
    opponent_board.rook &= !target_square;
    opponent_board.queen &= !target_square;
    opponent_board.king &= !target_square;

    // Return the updated board
    board
}

pub fn all_possible_moves(board: &ColorBoard, opponent_board: &ColorBoard, color: &Color) -> u64 {
    king_moves(board.king, board.bitboard())
        | queen_moves(board.queen, board.bitboard(), opponent_board.bitboard())
        | rooks_moves(board.rook, board.bitboard(), opponent_board.bitboard())
        | bishops_moves(board.bishop, board.bitboard(), opponent_board.bitboard())
        | knight_moves(board.bishop, board.bitboard())
        | pawn_moves(
            board.pawn,
            board.bitboard(),
            opponent_board.bitboard(),
            &color,
        )
}

pub fn is_king_checked(
    king_bitboard: u64,
    board: &ColorBoard,
    opponent_board: &ColorBoard,
    color: &Color,
) -> bool {
    king_bitboard & all_possible_moves(&board, &opponent_board, &color) != 0
}
