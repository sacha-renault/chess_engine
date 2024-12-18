use super::{color::Color, static_positions};

/// Performs a bitwise left shift operation on a 64-bit unsigned integer.
///
/// # Arguments
///
/// * `x` - The 64-bit unsigned integer to be shifted
/// * `y` - The number of positions to shift the bits to the left
///
/// # Returns
///
/// A 64-bit unsigned integer with bits shifted left by `y` positions
fn lshift(x: u64, y: u64) -> u64 {
    x << y
}

/// Performs a bitwise right shift operation on a 64-bit unsigned integer.
///
/// # Arguments
///
/// * `x` - The 64-bit unsigned integer to be shifted
/// * `y` - The number of positions to shift the bits to the right
///
/// # Returns
///
/// A 64-bit unsigned integer with bits shifted right by `y` positions
fn rshift(x: u64, y: u64) -> u64 {
    x >> y
}

/// Computes legal moves along a ray in a specified direction.
///
/// This function shifts a piece's position bitboard repeatedly, adding valid moves while stopping
/// at same-color pieces (blockage) or opponent pieces (capture).
///
/// # Parameters
/// - `piece_bitboard`: The bitboard of the piece's position.
/// - `direction`: `true` for left/upward shifts, `false` for right/downward shifts.
/// - `shift_value`: Number of bits to shift for the ray (e.g., 1 for horizontal, 8 for vertical).
/// - `same_color_bitboard`: Bitboard of same-color pieces.
/// - `other_color_bitboard`: Bitboard of opponent pieces.
///
/// # Returns
/// A `u64` bitboard representing valid moves along the ray.
fn ray_scanning(
    piece_bitboard: u64,
    direction: bool,
    shift_value: u64,
    same_color_bitboard: u64,
    other_color_bitboard: u64,
) -> u64 {
    let mut moves = 0u64;
    let mut ray = piece_bitboard;

    // Choose the operation based on direction
    let shift_fn: fn(u64, u64) -> u64 = if direction { lshift } else { rshift };

    // Define mask to prevent wrapping and constrain movement
    let file_mask = match (direction, shift_value) {
        // Horizontal movement (shift of 1)
        (true, 1) => !static_positions::FILE_H,
        (false, 1) => !static_positions::FILE_A,

        // Vertical movement (shift of 8)
        (true, 8) => !static_positions::RANK1,
        (false, 8) => !static_positions::RANK8,

        // Diagonal movements
        (true, 7) => !static_positions::FILE_H | !static_positions::RANK1,
        (false, 7) => !static_positions::FILE_A | !static_positions::RANK8,
        (true, 9) => !static_positions::FILE_A | !static_positions::RANK1,
        (false, 9) => !static_positions::FILE_H | !static_positions::RANK8,

        // Default case (should not occur with current usage)
        _ => 0xFFFFFFFFFFFFFFFF,
    };

    // Initial shift and mask
    ray = shift_fn(ray, shift_value) & file_mask;

    while ray != 0 {
        // If the same color piece is encountered, stop the scan in that direction
        if same_color_bitboard & ray != 0 {
            break;
        }

        // If an opponent's piece is encountered, stop and add it to the moves (capture)
        if other_color_bitboard & ray != 0 {
            moves |= ray;
            break;
        }

        // Collect the current ray positions
        moves |= ray;

        // Continue scanning with file masking to prevent wrapping
        ray = shift_fn(ray, shift_value) & file_mask;
    }

    moves
}

/// Computes all possible moves for a rook on the chessboard.
///
/// # Arguments
///
/// * `rook_bitboard` - Bitboard representing the rook's current position
/// * `same_color_bitboard` - Bitboard of all pieces of the same color
/// * `other_color_bitboard` - Bitboard of all pieces of the opposite color
///
/// # Returns
///
/// A bitboard representing all valid moves for the rook
///
/// # Notes
///
/// Calculates moves in four directions:
/// - Vertical downward (false, 8)
/// - Vertical upward (true, 8)
/// - Horizontal leftward (false, 1)
/// - Horizontal rightward (true, 1)
pub fn rooks_moves(rook_bitboard: u64, same_color_bitboard: u64, other_color_bitboard: u64) -> u64 {
    ray_scanning(
        rook_bitboard,
        false,
        8,
        same_color_bitboard,
        other_color_bitboard,
    ) | ray_scanning(
        rook_bitboard,
        true,
        8,
        same_color_bitboard,
        other_color_bitboard,
    ) | ray_scanning(
        rook_bitboard,
        false,
        1,
        same_color_bitboard,
        other_color_bitboard,
    ) | ray_scanning(
        rook_bitboard,
        true,
        1,
        same_color_bitboard,
        other_color_bitboard,
    )
}

/// Computes all possible moves for a bishop on the chessboard.
///
/// # Arguments
///
/// * `bishop_bitboard` - Bitboard representing the bishop's current position
/// * `same_color_bitboard` - Bitboard of all pieces of the same color
/// * `other_color_bitboard` - Bitboard of all pieces of the opposite color
///
/// # Returns
///
/// A bitboard representing all valid moves for the bishop
pub fn bishops_moves(
    bishop_bitboard: u64,
    same_color_bitboard: u64,
    other_color_bitboard: u64,
) -> u64 {
    ray_scanning(
        bishop_bitboard,
        false,
        7,
        same_color_bitboard,
        other_color_bitboard,
    ) | ray_scanning(
        bishop_bitboard,
        true,
        7,
        same_color_bitboard,
        other_color_bitboard,
    ) | ray_scanning(
        bishop_bitboard,
        false,
        9,
        same_color_bitboard,
        other_color_bitboard,
    ) | ray_scanning(
        bishop_bitboard,
        true,
        9,
        same_color_bitboard,
        other_color_bitboard,
    )
}

/// equivalent to `bishops_moves() | rooks_move()`
pub fn queen_moves(
    queen_bitboard: u64,
    same_color_bitboard: u64,
    other_color_bitboard: u64,
) -> u64 {
    rooks_moves(queen_bitboard, same_color_bitboard, other_color_bitboard)
        | bishops_moves(queen_bitboard, same_color_bitboard, other_color_bitboard)
}

/// Computes all possible moves for a pawn on the chessboard.
///
/// # Arguments
///
/// * `pawn_bitboard` - Bitboard representing the pawn's current position
/// * `same_color_bitboard` - Bitboard of all pieces of the same color
/// * `other_color_bitboard` - Bitboard of all pieces of the opposite color
/// * `color` - Color of the pawn (determines movement direction)
///
/// # Returns
///
/// A bitboard representing all valid moves for the pawn, including:
/// - Single square push
/// - Double square push (from starting rank)
/// - Left and right diagonal captures
pub fn pawn_moves(
    pawn_bitboard: u64,
    same_color_bitboard: u64,
    other_color_bitboard: u64,
    color: &Color,
) -> u64 {
    let empty_squares = !(same_color_bitboard | other_color_bitboard);
    let shift_fn = match color {
        Color::Black => rshift,
        Color::White => lshift,
    };
    let double_move_rank = match color {
        Color::Black => static_positions::RANK6,
        Color::White => static_positions::RANK3,
    };
    let (file1, file2) = match color {
        Color::Black => (static_positions::FILE_H, static_positions::FILE_A),
        Color::White => (static_positions::FILE_A, static_positions::FILE_H),
    };

    // Single push for white pawns
    let single_push = shift_fn(pawn_bitboard, 8) & empty_squares;

    // Double push (only from the second rank for white pawns)
    let double_push = shift_fn(single_push & double_move_rank, 8) & empty_squares;

    // Left capture (diagonal capture to the left for white pawns)
    let left_capture = shift_fn(pawn_bitboard, 7) & other_color_bitboard & !file1;

    // Right capture (diagonal capture to the right for white pawns)
    let right_capture = shift_fn(pawn_bitboard, 9) & other_color_bitboard & !file2;

    single_push | double_push | left_capture | right_capture
}

/// Computes all possible moves for knights on the chessboard.
///
/// # Arguments
///
/// * `knights_bitboard` - Bitboard representing the positions of all knights
/// * `same_color_bitboard` - Bitboard of all pieces of the same color
///
/// # Returns
///
/// A bitboard representing all valid moves for the knights
///
/// # Notes
///
/// Uses precomputed move sets and iterates through all knights on the board
pub fn knight_moves(knights_bitboard: u64, same_color_bitboard: u64) -> u64 {
    let mut moves = 0u64;
    let mut knights = knights_bitboard; // Copy of knights bitboard
    while knights != 0 {
        // Get the index of the least significant set bit
        let square_index = knights.trailing_zeros() as usize;

        // Add the precomputed knight moves for this square, masking out same-color pieces
        moves |= static_positions::KNIGHTS_MOVES[square_index] & !same_color_bitboard;

        // Remove the processed knight from the bitboard
        knights &= knights - 1;
    }

    moves
}

/// Computes all possible moves for a king on the chessboard.
///
/// # Arguments
///
/// * `king_bitboard` - Bitboard representing the king's current position
/// * `same_color_bitboard` - Bitboard of all pieces of the same color
///
/// # Returns
///
/// A bitboard representing all valid moves for the king
///
/// # Notes
///
/// Uses a precomputed move set for the king's current square
pub fn king_moves(king_bitboard: u64, same_color_bitboard: u64) -> u64 {
    static_positions::KING_MOVES[king_bitboard.trailing_zeros() as usize] & !same_color_bitboard
}
