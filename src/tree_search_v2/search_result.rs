use crate::prelude::PlayerMove;

pub struct SearchResult {
    pv: Vec<PlayerMove>,
    score: f32,
    depth: usize,
}

impl SearchResult {
    pub fn new(pv: Vec<PlayerMove>, score: f32, depth: usize) -> Self {
        Self { pv, score, depth }
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

    pub fn max_depth(&self) -> usize {
        self.pv.len()
    }
}
