use std::fmt;

use crate::prelude::PlayerMove;

#[derive(Debug)]
pub struct SearchResult {
    pv: Vec<PlayerMove>,
    score: f32,
    depth: usize,
    tree_max_depth: usize,
    node_count: usize,
}

impl SearchResult {
    pub fn new(
        pv: Vec<PlayerMove>,
        score: f32,
        depth: usize,
        tree_max_depth: usize,
        node_count: usize,
    ) -> Self {
        Self {
            pv,
            score,
            depth,
            tree_max_depth,
            node_count,
        }
    }

    pub fn best_move(&self) -> &PlayerMove {
        &self.pv[0]
    }

    pub fn principale_variation(&self) -> &Vec<PlayerMove> {
        &self.pv
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn qdepth(&self) -> usize {
        self.pv.len()
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the principal variation
        let pv_str = if self.pv.is_empty() {
            "None".to_string()
        } else {
            self.pv
                .iter()
                .map(|m| format!("{}", m)) // Assumes PlayerMove implements Display
                .collect::<Vec<_>>()
                .join(" > ")
        };

        write!(
            f,
            "Search Result:\n  Score: {:.2}\n  Depth/QDepth/MaxDepth: {}/{}/{}\n  Nodes: {}\n  PV: {}",
            self.score,
            self.depth,
            self.qdepth(),
            self.tree_max_depth,
            self.node_count,
            pv_str
        )
    }
}
