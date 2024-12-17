use super::{board::Color, static_positions};

/// Compute bit lshift
fn lshift(x: u64, y: u64) -> u64 {
    x << y
}

/// Compute bit rshift
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
fn ray_scanning(piece_bitboard: u64, direction: bool, shift_value: u64, same_color_bitboard: u64, other_color_bitboard: u64) -> u64 {
    let mut moves = 0u64;
    let mut ray = piece_bitboard;

    // Choose the operation based on direction
    let shift_fn: fn(u64, u64) -> u64 = if direction { lshift } else { rshift };
    let file_mask = if direction { !static_positions::FILE_A } else { !static_positions::FILE_H };


    // Shift ray in the chosen direction
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

        // Continue scanning
        ray = shift_fn(ray, shift_value) & file_mask;
    }

    moves
}

/// Compute all rooks move available from the bitboard
pub fn rooks_moves(rook_bitboard: u64, same_color_bitboard: u64, other_color_bitboard: u64) -> u64 {
    ray_scanning(rook_bitboard, false, 8, same_color_bitboard, other_color_bitboard)
    | ray_scanning(rook_bitboard, true, 8, same_color_bitboard, other_color_bitboard)
    | ray_scanning(rook_bitboard, false, 1, same_color_bitboard, other_color_bitboard)
    | ray_scanning(rook_bitboard, true, 1, same_color_bitboard, other_color_bitboard)
}

pub fn bishops_moves(bishop_bitboard: u64, same_color_bitboard: u64, other_color_bitboard: u64) -> u64 {
    ray_scanning(bishop_bitboard, false, 7, same_color_bitboard, other_color_bitboard)
    | ray_scanning(bishop_bitboard, true, 7, same_color_bitboard, other_color_bitboard)
    | ray_scanning(bishop_bitboard, false, 9, same_color_bitboard, other_color_bitboard)
    | ray_scanning(bishop_bitboard, true, 9, same_color_bitboard, other_color_bitboard)
}

pub fn queen_moves(queen_bitboard: u64, same_color_bitboard: u64, other_color_bitboard: u64) -> u64 {
    rooks_moves(queen_bitboard, same_color_bitboard, other_color_bitboard)
    | bishops_moves(queen_bitboard, same_color_bitboard, other_color_bitboard)
}

pub fn pawn_moves(pawn_bitboard: u64, same_color_bitboard: u64, other_color_bitboard: u64, color: Color) -> u64 {
    let empty_squares = !(same_color_bitboard | other_color_bitboard);
    let shift_fn = match color {
        Color::Black => rshift,
        Color::White => lshift
    };
    let double_move_rank = match color {
        Color::Black => static_positions::RANK6,
        Color::White => static_positions::RANK3
    };
    let (file1, file2) = match color {
        Color::Black => (static_positions::FILE_H, static_positions::FILE_A),
        Color::White => (static_positions::FILE_A, static_positions::FILE_H)
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

pub fn king_moves(king_bitboard: u64, same_color_bitboard: u64) -> u64 {
    static_positions::KING_MOVES[king_bitboard.trailing_zeros() as usize] & !same_color_bitboard
}