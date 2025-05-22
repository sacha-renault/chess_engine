use crate::prelude::{string_from_move, Engine, PlayerMove};
use crate::static_evaluation::evaluator_trait::Evaluator;

use super::tree_node::NodeHandle;
use super::tree_node_pool::TreeNodePool;

pub struct TreeSearch {
    pool: TreeNodePool,
    evaluator: Box<dyn Evaluator>,
}

impl TreeSearch {
    pub fn new(pool_capacity: usize, evaluator: Box<dyn Evaluator>) -> Self {
        Self {
            pool: TreeNodePool::with_capacity(pool_capacity),
            evaluator,
        }
    }

    pub fn iterative_search(&mut self, position: Engine, depth: u8) -> Option<PlayerMove> {
        // Clear pool for new search
        self.pool.clear();

        // Create root node
        let root = self.pool.allocate_node(position, 0.0, None, None, None)?;

        // Init best move
        let mut best_move = None;

        // Iterative deepening
        for i_depth in 1..=depth {
            match self.negamax(root, i_depth, f32::NEG_INFINITY, f32::INFINITY, 1) {
                Ok(_) => best_move = self.get_best_move(root),
                Err(_) => break,
            }
            println!(
                "Depth : {}, best move : {}",
                i_depth,
                string_from_move(&best_move.unwrap())
            )
        }

        best_move
    }

    pub fn search(&mut self, position: Engine, depth: u8) -> Option<PlayerMove> {
        // Clear pool for new search
        self.pool.clear();

        // Create root node
        let root = self.pool.allocate_node(position, 0.0, None, None, None)?;

        // Run negamax
        let _ = self.negamax(root, depth, f32::NEG_INFINITY, f32::INFINITY, 1);

        // Return best move from root's children
        self.get_best_move(root)
    }

    fn negamax(
        &mut self,
        node_handle: NodeHandle,
        depth: u8,
        mut alpha: f32,
        beta: f32,
        color: i8,
    ) -> Result<f32, ()> {
        if depth == 0 {
            // Return evaluation * color for negamax
            return Ok(self.pool.get_node(node_handle).ok_or(())?.get_score() * color as f32);
        }

        // Generate children if not done yet
        if !self
            .pool
            .get_node(node_handle)
            .unwrap()
            .has_children_computed()
        {
            self.generate_children(node_handle)?;
        }

        let mut best_score = f32::NEG_INFINITY;

        // Get children (need to clone to avoid borrow issues)
        let children = self
            .pool
            .get_node(node_handle)
            .ok_or(())?
            .get_children()
            .clone();

        for child_handle in children {
            let score = -self.negamax(child_handle, depth - 1, -beta, -alpha, -color)?;

            if score > best_score {
                best_score = score;
                // Update best move tracking
                self.pool
                    .get_node_mut(node_handle)
                    .unwrap()
                    .set_best_score(score);
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                break; // Alpha-beta pruning
            }
        }

        Ok(best_score)
    }

    /// Computes and adds all possible child nodes for a given position
    ///
    /// # Parameters
    /// * `handle` - Node handle for which to generate children
    ///
    /// # Note
    /// Also handles terminal positions (checkmate/stalemate)
    fn generate_children(&mut self, handle: NodeHandle) -> Result<(), ()> {
        // First, get the possible moves
        let (engine_copy, already_computed) = {
            let node = self.pool.get_node(handle).ok_or(())?;
            (node.get_engine(), node.has_children_computed())
        };

        // Early return if already computed
        if already_computed {
            return Ok(());
        }

        // Generate moves from the cloned engine
        let possible_moves = engine_copy
            .generate_moves_with_engine_state()
            .unwrap_or_default();

        // Collect child handles first
        let mut child_handles = Vec::new();

        // Create all child nodes
        for possible_move in possible_moves.into_iter() {
            // Calculate the raw score of this board
            let score = self
                .evaluator
                .evaluate_engine_state(&possible_move.engine, 0);

            // Put the node in the pool
            let child_handle = self
                .pool
                .allocate_node(
                    possible_move.engine,
                    score,
                    Some(possible_move.player_move),
                    Some(possible_move.piece),
                    possible_move.captured_piece,
                )
                .ok_or(())?;

            child_handles.push(child_handle);
        }

        // Now add all children to the parent node and mark as computed
        let node = self.pool.get_node_mut(handle).ok_or(())?;
        for child_handle in child_handles {
            node.add_child(child_handle);
        }
        node.set_computed(true);

        Ok(())
    }

    /// Gets the best move from the root node based on the search results
    ///
    /// # Parameters
    /// * `root_handle` - Handle to the root node of the search tree
    ///
    /// # Returns
    /// * `Some(PlayerMove)` - The best move found, or None if no moves available
    fn get_best_move(&self, root_handle: NodeHandle) -> Option<PlayerMove> {
        let root_node = self.pool.get_node(root_handle)?;

        // If no children, no moves available (game over)
        if root_node.get_children().is_empty() {
            return None;
        }

        // Get whos turn it is
        let white_to_play = root_node.get_engine().white_to_play();

        let mut best_move = None;
        let mut best_score = f32::NEG_INFINITY;

        // Iterate through all child nodes to find the one with the best score
        for &child_handle in root_node.get_children() {
            if let Some(child_node) = self.pool.get_node(child_handle) {
                // Get the score for this child (negated because we're looking from root's perspective)
                let child_score = if white_to_play {
                    -child_node.get_best_score()
                } else {
                    child_node.get_best_score()
                };

                if child_score > best_score {
                    best_score = child_score;
                    best_move = child_node.get_move().clone();
                }
            }
        }

        best_move
    }
}
