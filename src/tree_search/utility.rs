use super::tree_node::TreeNodeRef;
use crate::static_evaluation::values;

pub fn is_unstable_position(node: TreeNodeRef) -> bool {
    let borrowed = node.borrow();
    let engine = borrowed.get_engine();
    
    // Check if the move is a capture or gives check
    if engine.is_current_king_checked() {
        return true;
    }
    
    // Look for captures
    if let Some(_) = borrowed.get_captured_piece() {
        return true;
    }
    
    // TODO
    // More unstability
    false
}

// Helper function to adjust scores based on depth
pub fn adjust_score_for_depth(score: f32, depth: usize) -> f32 {
    if score.abs() >= values::VALUE_TB_WIN_IN_MAX_PLY {
        // If it's a checkmate score, adjust it based on depth
        // The deeper the depth, the less valuable the checkmate becomes
        if score > 0.0 {
            // For positive scores (winning), earlier mates are better
            score - depth as f32
        } else {
            // For negative scores (losing), later mates are better
            score + depth as f32
        }
    } else {
        // For non-checkmate scores, return as is
        score
    }
}
