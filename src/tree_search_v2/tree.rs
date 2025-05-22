use crate::prelude::{string_from_move, Engine, PlayerMove};
use crate::static_evaluation::evaluator_trait::Evaluator;
use crate::static_evaluation::values;

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

        // Get if it's white turn or black
        let color = if position.white_to_play() { 1 } else { -1 };

        // Create root node
        let root = self.pool.allocate_node(position, 0.0, None, None, None)?;

        // Init best move
        let mut best_move = None;

        // Iterative deepening
        for i_depth in 1..=depth {
            match self.negamax(root, i_depth, f32::NEG_INFINITY, f32::INFINITY, color) {
                Ok(_) => best_move = self.get_best_move(root),
                Err(_) => break,
            }
            println!(
                "Depth : {}, best move : {}",
                i_depth,
                string_from_move(
                    &best_move.expect(&format!("??? Why does it crashes ? : {}", i_depth))
                )
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
            return self.quiescence_search(node_handle, -beta, -alpha, -color);
        }

        // Generate children if not done yet
        if !self
            .pool
            .get_node(node_handle)
            .expect("`negamax` need a valid node handle")
            .has_children_computed()
        {
            self.generate_children(node_handle)?;
        }

        let mut best_score = f32::NEG_INFINITY;

        // Get children
        let children = self.get_children_sorted_by_score(node_handle)?;

        // If there is no children at all, it means it's a terminal node
        // We can return instant the best score!
        if children.is_empty() {
            return Ok(self
                .pool
                .get_node(node_handle)
                .expect("`negamax` need a valid node handle")
                .get_best_score());
        }

        // Iterate over the child
        for child_handle in children {
            let score = -self.negamax(child_handle, depth - 1, -beta, -alpha, -color)?;

            if score > best_score {
                best_score = score;
                // Update best move tracking
                self.pool
                    .get_node_mut(node_handle)
                    .expect("`negamax` children need a valid node handle")
                    .set_best_score(score);
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                break; // Alpha-beta pruning
            }
        }

        Ok(best_score)
    }

    fn quiescence_search(
        &mut self,
        node_handle: NodeHandle,
        mut alpha: f32,
        beta: f32,
        color: i8,
    ) -> Result<f32, ()> {
        // Stand pat evaluation
        let stand_pat = self.pool.get_node(node_handle).ok_or(())?.get_score();

        if stand_pat >= beta {
            return Ok(beta);
        }

        alpha = alpha.max(stand_pat);

        // Generate only tactical moves (captures, checks, maybe promotions)
        // Generate children if not done yet
        if !self
            .pool
            .get_node(node_handle)
            .unwrap()
            .has_children_computed()
        {
            // TODO, we want to generate only tactical moves here
            self.generate_children(node_handle)?;
        }

        // Get children (need to clone to avoid borrow issues)
        let children = self.get_children_sorted_by_score(node_handle)?;
        let mut best_score = stand_pat;

        for child_handle in children {
            if self.is_tactical_node(child_handle) {
                let score = -self.quiescence_search(child_handle, -beta, -alpha, -color)?;

                best_score = best_score.max(score);
                alpha = alpha.max(score);

                if alpha >= beta {
                    break;
                }
            }
        }

        Ok(best_score)
    }

    fn is_tactical_node(&self, handle: NodeHandle) -> bool {
        let node = self
            .pool
            .get_node(handle)
            .expect("`is_tactical_node` needs a valid handle");

        // Check if the move is a capture or gives check
        if node.get_engine().is_current_king_checked() {
            true
        } else if node.get_captured_piece().is_some() {
            true
        } else if matches!(node.get_move(), Some(PlayerMove::Promotion(_))) {
            true
        } else {
            false
        }
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

        // If there is no possible moves, it means it's either stalemate
        // or checkmate
        if possible_moves.is_empty() {
            self.evaluate_terminal_node(handle);
            return Ok(());
        }

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

    /// Evaluates a terminal node (checkmate or stalemate)
    ///
    /// # Parameters
    /// * `node` - Terminal node to evaluate
    ///
    /// # Returns
    /// Score for the terminal position (0 for stalemate, CHECK_MATE_VALUE for checkmate)
    fn evaluate_terminal_node(&mut self, handle: NodeHandle) -> f32 {
        // Get the node
        let node = self
            .pool
            .get_node_mut(handle)
            .expect("`evaluate_terminal_node` needs a valid handle");

        // if it's terminal node (number of moves == 0)
        // it means it's either check mate or stale mate
        if node.get_engine().is_current_king_checked() {
            let score = -values::CHECK_MATE;
            node.set_score(score);
            node.set_best_score(score);
            score
        } else {
            // This is a stalemate case
            node.set_score(0.);
            node.set_best_score(0.);
            0.
        }
    }

    /// Returns sorted children nodes with their evaluation scores
    ///
    /// # Parameters
    /// * `node` - Parent node whose children to sort
    /// * `shallow_depth` - Depth for preliminary evaluation
    ///
    /// # Returns
    /// Vector of node handles sorted by there score
    fn get_children_sorted_by_score(&self, handle: NodeHandle) -> Result<Vec<NodeHandle>, ()> {
        let children = self.pool.get_node(handle).ok_or(())?.get_children().clone();

        let mut scored_children = children
            .iter()
            .map(|&child_handle| {
                let child = self.pool.get_node(child_handle).ok_or(())?;
                let base_score = child.get_score();

                let player_move = child.get_move().ok_or(())?;
                let moved_piece = child.get_moved_piece();
                let captured_piece_opt = child.get_captured_piece();
                let is_king_checked = child.get_engine().is_current_king_checked();

                let bonus = self.evaluator.evaluate_heuristic_move(
                    player_move,
                    moved_piece,
                    captured_piece_opt,
                    is_king_checked,
                ) * values::HEURISTIC_WEIGHT;

                Ok((child_handle, base_score + bonus))
            })
            .collect::<Result<Vec<_>, ()>>()?;

        scored_children.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        Ok(scored_children
            .into_iter()
            .map(|(handle, _)| handle)
            .collect())
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

        let mut best_move = None;
        let mut best_score = f32::NEG_INFINITY;

        // Iterate through all child nodes to find the one with the best score
        for child_handle in self.get_children_sorted_by_score(root_handle).ok()? {
            if let Some(child_node) = self.pool.get_node(child_handle) {
                // Get the score for this child (negated because we're looking from root's perspective)
                let child_score = -child_node.get_best_score();

                // println!(
                //     "For move : {} - Best score : {}",
                //     string_from_move(&child_node.get_move().unwrap()),
                //     child_score
                // );

                if child_score > best_score {
                    best_score = child_score;
                    best_move = child_node.get_move().clone();
                }
            }
        }

        println!("Best mv score : {}", best_score);
        best_move
    }
}
