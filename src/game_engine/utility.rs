use super::move_piece_output::PieceMoveOutput;
use super::player_move::{CastlingMove, NormalMove, PlayerMove};
use crate::boards::Board;
use crate::boards::ColorBoard;
use crate::pieces::static_positions::{
    BLACK_KING, BLACK_ROOKS, FILE_A, FILE_H, WHITE_KING, WHITE_ROOKS,
};
use crate::pieces::Color;
use crate::pieces::Piece;
use crate::pieces::{moves::*, static_positions};

/// Returns the type of piece on a given square in the color board.
///
/// # Arguments
/// * `color_board` - A reference to the `ColorBoard`.
/// * `square` - A `u64` representing the square.
///
/// # Returns
/// An `Option<Pieces>` indicating the type of piece on the square.
pub fn get_piece_type(color_board: &ColorBoard, square: u64) -> Option<Piece> {
    if color_board.pawn & square != 0 {
        return Some(Piece::Pawn);
    }

    if color_board.knight & square != 0 {
        return Some(Piece::Knight);
    }

    if color_board.bishop & square != 0 {
        return Some(Piece::Bishop);
    }

    if color_board.rook & square != 0 {
        return Some(Piece::Rook);
    }

    if color_board.queen & square != 0 {
        return Some(Piece::Queen);
    }

    if color_board.king & square != 0 {
        return Some(Piece::King);
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

/// Returns an iterator over the positions of set bits (1s) in a `u64` value.
///
/// The iterator yields the positions of the bits that are set to 1, starting from
/// the least significant bit (position 0).
///
/// # Arguments
///
/// * `value` - A `u64` value to extract the set bit positions from.
///
/// # Returns
///
/// An iterator that yields `u32` values representing the positions of the set bits.
pub fn iter_into_u64(mut value: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        if value == 0 {
            None
        } else {
            let pos = value.trailing_zeros() as u64;
            value &= value - 1;
            Some(pos)
        }
    })
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
    piece: Piece,
    start_square: u64,
    same_color_bitboard: u64,
    other_color_bitboard: u64,
    en_passant_squares: u64,
    color: Color,
) -> u64 {
    let bitboard = start_square & same_color_bitboard;
    match piece {
        Piece::King => king_moves(bitboard, same_color_bitboard),
        Piece::Queen => queen_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Piece::Rook => rooks_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Piece::Knight => knight_moves(bitboard, same_color_bitboard),
        Piece::Bishop => bishops_moves(bitboard, same_color_bitboard, other_color_bitboard),
        Piece::Pawn => pawn_moves(
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
    piece_type: Piece,
) -> PieceMoveOutput {
    // Determine which color's board is being modified
    let (color_board, opponent_board) = match color {
        Color::White => (&mut board.white, &mut board.black),
        Color::Black => (&mut board.black, &mut board.white),
    };

    // Determine captured piece
    let captured_piece = [
        (Piece::Pawn, opponent_board.pawn),
        (Piece::Knight, opponent_board.knight),
        (Piece::Bishop, opponent_board.bishop),
        (Piece::Rook, opponent_board.rook),
        (Piece::Queen, opponent_board.queen),
        (Piece::King, opponent_board.king),
    ]
    .into_iter()
    .find(|&(_, bitboard)| bitboard & target_square != 0)
    .map(|(piece, _)| piece);

    // Clear the current square from the moving piece
    match piece_type {
        Piece::Pawn => {
            color_board.pawn &= !current_square;
            color_board.pawn |= target_square;
        }
        Piece::Knight => {
            color_board.knight &= !current_square;
            color_board.knight |= target_square;
        }
        Piece::Bishop => {
            color_board.bishop &= !current_square;
            color_board.bishop |= target_square;
        }
        Piece::Rook => {
            color_board.rook &= !current_square;
            color_board.rook |= target_square;
        }
        Piece::Queen => {
            color_board.queen &= !current_square;
            color_board.queen |= target_square;
        }
        Piece::King => {
            color_board.king &= !current_square;
            color_board.king |= target_square;
        }
    }

    // Clear the target square from all opponent pieces
    match captured_piece {
        Some(Piece::Pawn) => opponent_board.pawn &= !target_square,
        Some(Piece::Knight) => opponent_board.knight &= !target_square,
        Some(Piece::Bishop) => opponent_board.bishop &= !target_square,
        Some(Piece::Rook) => opponent_board.rook &= !target_square,
        Some(Piece::Queen) => opponent_board.queen &= !target_square,
        Some(Piece::King) => opponent_board.king &= !target_square,
        None => (),
    }

    // Return the updated board
    PieceMoveOutput {
        board,
        captured_piece,
    }
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
        Color::White => (WHITE_KING, WHITE_ROOKS & FILE_H, WHITE_ROOKS & FILE_A),
        Color::Black => (BLACK_KING, BLACK_ROOKS & FILE_H, BLACK_ROOKS & FILE_A),
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

pub fn is_promotion_available(board: &Board, target_square: u64, color: Color) -> bool {
    let (player_board, _) = get_half_turn_boards(board, color);
    player_board.pawn & get_promotion_rank_by_color(color) & target_square != 0
}

pub fn create_normal_move(current_square: u64, target_square: u64) -> PlayerMove {
    PlayerMove::Normal(NormalMove::new(current_square, target_square))
}

pub fn create_move_from_str(str_move: &str) -> PlayerMove {
    if str_move == "O-O" {
        PlayerMove::Castling(CastlingMove::Short)
    } else if str_move == "O-O-O" {
        PlayerMove::Castling(CastlingMove::Long)
    } else if str_move.len() == 4 {
        // Parse a regular move like "e2e4"
        let chars: Vec<char> = str_move.chars().collect();
        let current_file = chars[0];
        let current_rank = chars[1];
        let target_file = chars[2];
        let target_rank = chars[3];

        // Validate the input
        if !('a'..='h').contains(&current_file)
            || !('1'..='8').contains(&current_rank)
            || !('a'..='h').contains(&target_file)
            || !('1'..='8').contains(&target_rank)
        {
            panic!("Invalid chess move notation: {}", str_move);
        }

        // Convert file and rank to bitboard positions
        let current_bitboard =
            1u64 << ((current_rank as u64 - '1' as u64) * 8 + (current_file as u64 - 'a' as u64));
        let target_bitboard =
            1u64 << ((target_rank as u64 - '1' as u64) * 8 + (target_file as u64 - 'a' as u64));

        // Create a normal move
        PlayerMove::Normal(NormalMove::new(current_bitboard, target_bitboard))
    } else {
        panic!("Invalid move format: {}", str_move);
    }
}

pub fn string_from_move(player_move: &PlayerMove) -> String {
    match player_move {
        PlayerMove::Castling(castling_move) => match castling_move {
            CastlingMove::Short => String::from("O-O"),
            CastlingMove::Long => String::from("O-O-O"),
        },
        PlayerMove::Normal(normal_move) => {
            let (current, target) = normal_move.squares();
            let (current_rank, current_file) = u64_to_coordinates(current);
            let (target_rank, target_file) = u64_to_coordinates(target);

            // Convert coordinates to chess notation
            format!(
                "{}{}{}{}",
                ((b'a' + current_file as u8) as char),
                ((b'1' + current_rank as u8) as char),
                ((b'a' + target_file as u8) as char),
                ((b'1' + target_rank as u8) as char)
            )
        }
        _ => "PROMOTION".to_string(),
    }
}
