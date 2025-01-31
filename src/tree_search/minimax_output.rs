use crate::game_engine::player_move::PlayerMove;

use super::tree_node::TreeNodeRef;

pub enum SearchOutput {
    Valid {
        best_node: Option<TreeNodeRef>,
        score: f32,
    },
    Invalid
    
}

impl SearchOutput {
    pub fn new(best_node: Option<TreeNodeRef>, score: f32) -> Self {
        SearchOutput::Valid {
            best_node: best_node,
            score,
        }
    }

    pub fn new_invalid() -> Self {
        SearchOutput::Invalid { }
    }

    pub fn get_move(&self) -> Option<PlayerMove> {
        match &self {
            SearchOutput::Invalid => None,
            SearchOutput::Valid { best_node,  .. } => match best_node {
                Some(node) => node.borrow().get_move().clone(),
                None => None,
            }
        }
    }

    pub fn get_score(&self) -> f32 {
        match &self {
            SearchOutput::Invalid => 0.,
            SearchOutput::Valid { score, .. } => *score
        }
    }

    pub fn get_depth(&self) -> usize {
        match &self {
            SearchOutput::Invalid => 0,
            SearchOutput::Valid { best_node,  .. } => match best_node {
                Some(node) => node.borrow().get_depth(),
                None => 0
            }
        }
    }

    pub fn mate_depth(&self) -> Option<usize> {
        match &self {
            SearchOutput::Invalid => None,
            SearchOutput::Valid { best_node,  .. } => match best_node {
                Some(node) => node.borrow().get_plies_to_mate(),
                None => None
            }
        }
    }

    pub fn node(&self) -> Option<TreeNodeRef> {
        match &self {
            SearchOutput::Invalid => None,
            SearchOutput::Valid { best_node,  .. } => match best_node {
                Some(node) => Some(node.clone()),
                None => None
            }
        }
    }
}