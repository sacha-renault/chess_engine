use super::tree_node::TreeNodeRef;

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
