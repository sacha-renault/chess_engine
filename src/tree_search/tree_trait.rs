use std::usize;
use super::minimax_output::SearchOutput;
use super::node_with_score::NodeWithScore;
use super::tree_node::TreeNodeRef;

// Core search functionality trait
pub trait SearchEngine {
    fn search_best_move(&mut self) -> SearchOutput;
    fn search(&mut self, node: TreeNodeRef, depth: usize, alpha: f32, beta: f32) -> SearchOutput;
    fn quiescence_search(&mut self, node: TreeNodeRef, alpha: f32, beta: f32, qdepth: usize, max_qdepth: usize) -> SearchOutput;
}
// Move ordering functionality
pub trait MoveOrderer {
    fn get_ordered_moves(&mut self, node: TreeNodeRef) -> Vec<NodeWithScore>;
}