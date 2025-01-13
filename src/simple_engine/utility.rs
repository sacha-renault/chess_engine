use super::board::Board;
use super::color::Color;
use super::color_board::ColorBoard;
use super::pieces::Pieces;
use super::player_move::CastlingMove;
use super::static_positions::{BLACK_KING, BLACK_ROOKS, FILE_A, FILE_H, WHITE_KING, WHITE_ROOKS};
use super::{moves::*, static_positions};

/// Returns the type of piece on a given square in the color board.
///
/// # Arguments
/// * `color_board` - A reference to the `ColorBoard`.
/// * `square` - A `u64` representing the square.
///
/// # Returns
/// An `Option<Pieces>` indicating the type of piece on the square.
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

/// Converts board coordinates to a bitboard representation.
///
/// # Arguments
/// * `position` - A tuple `(usize, usize)` representing the row and column.
///
/// # Returns
/// A `u64` representing the bitboard position.
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

/// Converts a bitboard representation to board coordinates.
///
/// # Arguments
/// * `bitboard` - A `u64` representing the bitboard position.
///
/// # Returns
/// A tuple `(usize, usize)` representing the row and column.
pub fn u64_to_coordinates(bitboard: u64) -> (usize, usize) {
    assert_eq!(
        bitboard.count_ones(),
        1,
        "Bitboard must have exactly one bit set. Got: {}",
        bitboard.count_ones()
    );

    let square_index = bitboard.trailing_zeros() as usize;
    let row = square_index / 8;
    let col = square_index % 8;

    (row, col)
}

/// Returns the possible moves for a given piece.
///
/// # Arguments
/// * `piece` - A reference to the `Pieces` type.
/// * `start_square` - A `u64` representing the starting square.
/// * `same_color_bitboard` - A `u64` representing the bitboard of the same color.
/// * `other_color_bitboard` - A `u64` representing the bitboard of the other color.
/// * `color` - A reference to the `Color` type.
///
/// # Returns
/// A `u64` representing the possible moves.
pub fn get_possible_move(
    piece: Pieces,
    start_square: u64,
    same_color_bitboard: u64,
    other_color_bitboard: u64,
    en_passant_squares: u64,
    color: Color,
) -> u64 {
    let bitboard = start_square & same_color_bitboard;
    match piece {
        Pieces::King => king_moves(bitboard, same_color_bitboard),
        Pieces::Queen => queen_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Pieces::Rook => rooks_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Pieces::Knight => knight_moves(bitboard, same_color_bitboard),
        Pieces::Bishop => bishops_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Pieces::Pawn => pawn_moves(
            bitboard,
            same_color_bitboard,
            other_color_bitboard | en_passant_squares,
            color,
        ),
    }
}

/// Returns the color based on the turn.
///
/// # Arguments
/// * `white_turn` - A `bool` indicating if it's white's turn.
///
/// # Returns
/// A `Color` representing the current turn's color.
pub fn get_color(white_turn: bool) -> Color {
    if white_turn {
        Color::White
    } else {
        Color::Black
    }
}

/// Moves a piece from one square to another on the board.
///
/// # Arguments
/// * `board` - A `Board` instance.
/// * `current_square` - A `u64` representing the current square.
/// * `target_square` - A `u64` representing the target square.
/// * `color` - A reference to the `Color` type.
/// * `piece_type` - A reference to the `Pieces` type.
///
/// # Returns
/// An updated `Board` instance.
pub fn move_piece(
    mut board: Board,
    current_square: u64,
    target_square: u64,
    color: Color,
    piece_type: Pieces,
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

/// Returns all possible moves for a given color board.
///
/// # Arguments
/// * `board` - A reference to the `ColorBoard`.
/// * `opponent_board` - A reference to the `ColorBoard` of the opponent.
/// * `color` - A reference to the `Color` type.
///
/// # Returns
/// A `u64` representing all possible moves.
pub fn all_possible_moves(board: &ColorBoard, opponent_board: &ColorBoard, color: Color) -> u64 {
    king_moves(board.king, board.bitboard())
        | queen_moves(board.queen, board.bitboard(), opponent_board.bitboard())
        | rooks_moves(board.rook, board.bitboard(), opponent_board.bitboard())
        | bishops_moves(board.bishop, board.bitboard(), opponent_board.bitboard())
        | knight_moves(board.knight, board.bitboard())
        | pawn_moves(
            board.pawn,
            board.bitboard(),
            opponent_board.bitboard() | opponent_board.en_passant,
            color,
        )
}

/// Checks if the king is in check.
///
/// # Arguments
/// * `king_bitboard` - A `u64` representing the king's bitboard.
/// * `board` - A reference to the `ColorBoard`.
/// * `opponent_board` - A reference to the `ColorBoard` of the opponent.
/// * `color` - A reference to the `Color` type.
///
/// # Returns
/// A `bool` indicating if the king is in check.
pub fn is_king_checked(
    king_bitboard: u64,
    board: &ColorBoard,
    opponent_board: &ColorBoard,
    color: Color,
) -> bool {
    king_bitboard & all_possible_moves(&board, &opponent_board, color) != 0
}

/// Get the boards for the current player and the opponent
///
/// # Returns
///
/// * A tuple containing references to the current player's board and the opponent's board.
pub fn get_half_turn_boards(board: &Board, color: Color) -> (&ColorBoard, &ColorBoard) {
    match color {
        Color::White => (&board.white, &board.black),
        Color::Black => (&board.black, &board.white),
    }
}

/// Same as `get_half_turn_boards` but matable
pub fn get_half_turn_boards_mut(
    board: &mut Board,
    color: Color,
) -> (&mut ColorBoard, &mut ColorBoard) {
    match color {
        Color::White => (&mut board.white, &mut board.black),
        Color::Black => (&mut board.black, &mut board.white),
    }
}

pub fn get_initial_castling_positions(color: Color) -> (u64, u64, u64) {
    match color {
        Color::White => (WHITE_KING, WHITE_ROOKS & FILE_A, WHITE_ROOKS & FILE_H),
        Color::Black => (BLACK_KING, BLACK_ROOKS & FILE_A, BLACK_ROOKS & FILE_H),
    }
}

pub fn get_final_castling_positions(castling: CastlingMove, color: Color) -> (u64, u64) {
    if castling == CastlingMove::Short {
        if color == Color::White {
            return (
                static_positions::WHITE_KING_SHORT_FINAL,
                static_positions::WHITE_ROOK_SHORT_FINAL,
            );
        } else {
            return (
                static_positions::BLACK_KING_SHORT_FINAL,
                static_positions::BLACK_ROOK_SHORT_FINAL,
            );
        }
    } else {
        if color == Color::White {
            return (
                static_positions::WHITE_KING_LONG_FINAL,
                static_positions::WHITE_ROOK_LONG_FINAL,
            );
        } else {
            return (
                static_positions::BLACK_KING_LONG_FINAL,
                static_positions::BLACK_ROOK_LONG_FINAL,
            );
        }
    }
}

pub fn get_required_empty_squares(castling: CastlingMove, color: Color) -> u64 {
    if castling == CastlingMove::Short {
        if color == Color::White {
            return static_positions::WHITE_SHORT_CASTLING_EMPTY;
        } else {
            return static_positions::BLACK_SHORT_CASTLING_EMPTY;
        }
    } else {
        if color == Color::White {
            return static_positions::WHITE_LONG_CASTLING_EMPTY;
        } else {
            return static_positions::BLACK_LONG_CASTLING_EMPTY;
        }
    }
}

pub fn get_promotion_rank_by_color(color: Color) -> u64 {
    match color {
        Color::White => static_positions::RANK8,
        Color::Black => static_positions::RANK1,
    }
}

pub fn get_en_passant_ranks(color: Color) -> u64 {
    if color == Color::White {
        return static_positions::RANK2 | static_positions::RANK4;
    } else {
        return static_positions::RANK7 | static_positions::RANK5;
    }
}
