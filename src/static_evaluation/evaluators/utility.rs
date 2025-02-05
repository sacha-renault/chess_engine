use crate::game_engine::player_move::PlayerMove;
use crate::pieces::{Color, Piece};
use crate::static_evaluation::values;

pub fn classic_heuristic_move_bonus(
    player_move: PlayerMove,
    moved_piece: Piece,
    captured_piece_opt: Option<Piece>,
    is_king_checked: bool,
) -> f32 {
    // init a bonus
    let mut bonus = 0.;

    // Some bonus for castling
    if let PlayerMove::Castling(_) = player_move {
        // Castling bonus is depth dependent
        // We wanna castle as soon as possible
        // TO protect the king
        bonus += values::CASTLING_BONUS;
    }

    // Some bonus for a promotion
    if let PlayerMove::Promotion(promotion_move) = player_move {
        // Promotion bonus is depth dependent
        // We wanna promote as soon as possible
        // To get a queen
        bonus += get_value_by_piece(promotion_move.promotion_piece());
    }

    // Some bonus for capturing a piece
    if let Some(captured_piece) = captured_piece_opt {
        bonus += values::CAPTURE_BONUS as f32;

        // Extra bonus for capturing a higher value piece
        let mvv_lva = get_value_by_piece(captured_piece) - get_value_by_piece(moved_piece);

        // if more than 0 we are capturing a higher value piece
        if mvv_lva > 0. {
            bonus += mvv_lva * values::CAPTURE_MVV_LVA_FACTOR;
        }
    }

    // If king is checked, we add a bonus
    if is_king_checked {
        bonus += values::CHECK_BONUS;
    }

    // TODO
    // We can keep adding more bonuses but i am really lazy to
    // Implement all, especially i think those are the most important
    bonus
}

pub fn get_value_multiplier_by_piece(piece: Piece, color: Color, bitboard: u64) -> f32 {
    let index = bitboard.trailing_zeros() as usize;
    match piece {
        Piece::Pawn => match color {
            Color::White => values::WHITE_PAWNS_VALUE[index],
            Color::Black => values::BLACK_PAWNS_VALUE[index],
        },
        Piece::Bishop => values::BISHOPS_VALUE[index],
        Piece::Knight => values::KNIGHTS_VALUE[index],
        _ => 1.,
    }
}

pub fn get_value_by_piece(piece: Piece) -> f32 {
    match piece {
        Piece::Pawn => 10.,
        Piece::Bishop => 30.,
        Piece::Knight => 30.,
        Piece::Rook => 50.,
        Piece::Queen => 90.,
        Piece::King => 0.,
    }
}