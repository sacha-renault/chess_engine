use std::fmt;

use crate::prelude::PlayerMove;

#[derive(Debug)]
pub struct SearchResult {
    best_move: PlayerMove,
    score: f32,
    depth: usize,
    tree_max_depth: usize,
    node_count: usize,
}

impl SearchResult {
    pub fn new(
        best_move: PlayerMove,
        score: f32,
        depth: usize,
        tree_max_depth: usize,
        node_count: usize,
    ) -> Self {
        Self {
            best_move,
            score,
            depth,
            tree_max_depth,
            node_count,
        }
    }

    pub fn best_move(&self) -> &PlayerMove {
        &self.best_move
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Search Result:\n  Score: {:.2}\n  Depth/MaxDepth: {}/{}\n  Nodes: {}\n  Best Move: {}",
            self.score, self.depth, self.tree_max_depth, self.node_count, self.best_move
        )
    }
}
