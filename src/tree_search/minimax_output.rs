use crate::game_engine::player_move::PlayerMove;

use super::tree_node::TreeNodeRef;

pub struct MinimaxOutput {
    best_node: Option<TreeNodeRef>,
    score: f32,
}

impl MinimaxOutput {
    pub fn new(best_node: Option<TreeNodeRef>, score: f32) -> MinimaxOutput {
        MinimaxOutput {
            best_node: best_node,
            score,
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
        match &self.best_node {
            Some(node) => node.borrow().get_depth(),
            None => 0
        }
    }

    pub fn mate_depth(&self) -> Option<usize> {
        match &self.best_node {
            Some(node) => node.borrow().get_plies_to_mate(),
            None => None
        }
    }

    pub fn node(&self) -> Option<TreeNodeRef> {
        match &self.best_node {
            Some(node) => Some(node.clone()),
            None => None
        }
    }
}