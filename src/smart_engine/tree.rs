use crate::prelude::{create_normal_move, iter_into_u64, Engine};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct TreeNode {
    engine: Engine,
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(engine: Engine) -> Self {
        TreeNode {
            engine,
            children: Vec::new(),
        }
    }

    pub fn create_root_node(engine: Engine) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(engine)))
    }

    pub fn generate_tree(node: Rc<RefCell<TreeNode>>, depth: usize) {
        if depth == 0 {
            return;
        }

        let current_engine = node.borrow().engine.clone();
        for piece_moves in current_engine.get_all_moves_by_piece().unwrap() {
            for possible_move in iter_into_u64(piece_moves.2) {
                let mut new_engine = current_engine.clone();
                let m = create_normal_move(piece_moves.0, 1 << possible_move);
                if new_engine.play(m).is_ok() {
                    let child_node = Rc::new(RefCell::new(TreeNode::new(new_engine)));
                    node.borrow_mut().children.push(child_node.clone());
                    TreeNode::generate_tree(child_node, depth - 1);
                }
            }
        }
    }

    pub fn get_tree_size(root_node: Rc<RefCell<TreeNode>>) -> u64 {
        let node = root_node.borrow();
        let mut size = 1; // Count current node

        // Recursively count children
        for child in node.children.iter() {
            size += TreeNode::get_tree_size(child.clone());
        }

        size
    }
}
