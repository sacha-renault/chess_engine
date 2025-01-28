use crate::game_engine::player_move::PlayerMove;

use super::tree_node::TreeNodeRef;

pub struct MinimaxOutput {
    best_node: Option<TreeNodeRef>,
    score: f32,
    depth: usize,
}

impl MinimaxOutput {
    pub fn new(best_node: Option<TreeNodeRef>, score: f32, depth: usize) -> MinimaxOutput {
        MinimaxOutput {
            best_node: best_node,
            score,
            depth,
        }
    }

    pub fn get_move(&self) -> Option<PlayerMove> {
        match &self.best_node {
            Some(node) => node.borrow().get_move().clone(),
            None => None,
        }
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn mate_depth(&self) -> Option<usize> {
        match &self.best_node {
            Some(node) => node.borrow().get_mate_depth(),
            None => None
        }
    }
}