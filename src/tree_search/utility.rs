use std::{collections::HashSet, rc::Rc};

use super::tree_node::{TreeNode, TreeNodeRef};
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

/// Calculates the total size of a tree starting from given root
///
/// # Parameters
/// * `root_node` - Starting node for size calculation
///
/// # Returns
/// Total number of unique nodes in the tree
pub fn get_tree_size(root_node: TreeNodeRef) -> usize {
    let mut visited = HashSet::new();
    get_tree_size_recursive(root_node, &mut visited)
}

/// Helper function for tree size calculation that handles cycles
///
/// # Parameters
/// * `root_node` - Current node being processed
/// * `visited` - Set of already visited nodes
///
/// # Returns
/// Number of unique nodes in this subtree
fn get_tree_size_recursive(
    root_node: TreeNodeRef,
    visited: &mut HashSet<*const std::cell::RefCell<TreeNode>>,
) -> usize {
    let raw_ptr = Rc::as_ptr(&root_node);

    // If this node has already been visited, return 0 to prevent double counting
    if !visited.insert(raw_ptr) {
        panic!("CYCLIC REFERENCES !!??");
    }

    let node = root_node.borrow();
    let mut size = 1; // Count current node

    // Recursively count children
    for child in node.get_children().iter() {
        size += get_tree_size_recursive(child.clone(), visited);
    }

    size
}

/// Returns initial score based on whether the player is maximizing
///
/// # Parameters
/// * `is_maximizing` - Whether the current player is maximizing
///
/// # Returns
/// Negative infinity for maximizing player, positive infinity for minimizing player
pub fn init_best_score(is_maximizing: bool) -> f32 {
    if is_maximizing {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    }
}

pub fn exceed_size_limit_prob(current_size: usize, max_size: usize) -> bool {
    if current_size * 10 > max_size {
        true
    } else {
        false
    }
}