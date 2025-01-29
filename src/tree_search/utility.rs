use super::tree_node::TreeNodeRef;

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
