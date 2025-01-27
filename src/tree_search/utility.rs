use super::tree_node::TreeNodeRef;
use super::values;
use super::values::get_value_by_piece;
use crate::game_engine::player_move::PlayerMove;
use crate::pieces::Piece;

pub fn heuristic_move_bonus_from_node(node: TreeNodeRef, is_white_to_play: bool) -> f32 {
    let node_ref = node.borrow();
    let player_move = node_ref.get_move().unwrap();
    let moved_piece = node_ref.get_moved_piece();
    let captured_piece_opt = node_ref.get_captured_piece();
    let is_king_checked = node_ref.get_engine().is_current_king_checked();
    heuristic_move_bonus(
        player_move,
        moved_piece,
        captured_piece_opt,
        1,
        is_king_checked,
        is_white_to_play,
    )
}

pub fn heuristic_move_bonus(
    player_move: PlayerMove,
    moved_piece: Piece,
    captured_piece_opt: Option<Piece>,
    depth: usize,
    is_king_checked: bool,
    white_to_play: bool,
) -> f32 {
    // init a bonus
    let mut bonus = 0.;

    // Some bonus for castling
    if let PlayerMove::Castling(_) = player_move {
        // Castling bonus is depth dependent
        // We wanna castle as soon as possible
        // TO protect the king
        bonus += values::CASTLING_BONUS * depth as f32;
    }

    // Some bonus for a promotion
    if let PlayerMove::Promotion(promotion_move) = player_move {
        // Promotion bonus is depth dependent
        // We wanna promote as soon as possible
        // To get a queen
        bonus += get_value_by_piece(promotion_move.promotion_piece()) * depth as f32;
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

    if white_to_play {
        bonus
    } else {
        -bonus
    }
}

pub fn is_unstable_position(node: TreeNodeRef) -> bool {
    let borrowed = node.borrow();
    if !borrowed.get_engine().is_current_king_checked() {
        false
    } else if borrowed.get_captured_piece().is_none() {
        false
    } else {
        true
    }
}
