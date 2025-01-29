use std::usize;

use crate::game_engine::player_move::PlayerMove;
use crate::game_engine::engine::Engine;

use super::minimax_output::MinimaxOutput;
use super::transposition_table::TTEntry;


// Core search functionality trait
pub trait SearchEngine {
    fn search(&mut self, position: &Engine, depth: usize) -> MinimaxOutput;
    fn quiescence_search(&mut self, position: &Engine, alpha: f32, beta: f32) -> f32;
}

// Move ordering functionality
pub trait MoveOrderer {
    fn order_moves(&self, moves: Vec<PlayerMove>, position: &Engine) -> Vec<PlayerMove>;
    fn score_move(&self, move_: &PlayerMove, position: &Engine) -> f32;
}

// Separate static evaluation
pub trait Evaluator {
    fn evaluate_position(&self, position: &Engine) -> f32;
    fn evaluate_tactical(&self, position: &Engine) -> f32;
}

// Transposition table operations
pub trait TranspositionTableManager {
    fn probe(&self, hash: u64) -> Option<TTEntry>;
    fn store(&mut self, hash: u64, entry: TTEntry);
    fn maintain(&mut self);
}