use crate::pieces::Piece;
use crate::game_engine::player_move::PlayerMove;
use super::values::get_value_by_piece;
use super::values;

pub fn heuristic_move_bonus(
    player_move: PlayerMove,
    moved_piece: Piece,
    captured_piece_opt: Option<Piece>,
    depth: usize,
    white_to_play: bool
) -> f32 {
    // init a bonus
    let mut bonus = 0.;

    // Some bonus for castling
    if let PlayerMove::Castling(_) = player_move {
        // Castling bonus is depth dependent
        // We wanna castle as soon as possible
        // TO protect the king
        bonus += values::CASTLING_BONUS * depth as f32;
    } else {
        // Some bonus for capturing a piece
        if let Some(captured_piece) = captured_piece_opt {
            bonus += values::CAPTURE_BONUS as f32;

            // Extra bonus for capturing a higher value piece
            let mvv_lva= get_value_by_piece(captured_piece) - get_value_by_piece(moved_piece);

            // if more than 0 we are capturing a higher value piece
            if mvv_lva > 0. {
                bonus += mvv_lva * values::CAPTURE_MVV_LVA_FACTOR;
            }
        }
    }

    if white_to_play {
        bonus
    } else {
        -bonus
    }
}