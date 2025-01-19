use super::tree_node::TreeNodeRef;
use crate::prelude::PlayerMove;

#[derive(Debug, Clone)]
pub struct NodeWithScore {
    node: TreeNodeRef,
    score: f32,
}

impl NodeWithScore {
    pub fn new(node: TreeNodeRef, score: f32) -> Self {
        NodeWithScore { node, score }
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn node(&self) -> TreeNodeRef {
        self.node.clone()
    }

    pub fn get_move(&self) -> PlayerMove {
        self.node.borrow().get_move().unwrap()
    }
}
