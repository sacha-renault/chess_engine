use derive_builder::Builder;

use crate::pieces::Piece;
use crate::prelude::evaluators::utility::get_value_by_piece;
use crate::prelude::{Engine, PlayerMove};
use crate::static_evaluation::evaluator_trait::Evaluator;
use crate::static_evaluation::values;

use super::search_result::SearchResult;
use super::tree_node::NodeHandle;
use super::tree_node_pool::TreeNodePool;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct TreeSearch {
    pool: TreeNodePool,
    evaluator: Box<dyn Evaluator>,
    max_depth: usize,
    max_q_depth: usize,
}

impl TreeSearchBuilder {
    pub fn pool_capacity(mut self, capacity: usize) -> Self {
        self.pool = Some(TreeNodePool::with_capacity(capacity));
        self
    }
}

impl TreeSearch {
    pub fn iterative_search(&mut self, position: Engine) -> Option<SearchResult> {
        // Clear pool for new search
        self.pool.clear();

        // Create root node
        let root = self.pool.allocate_node(position, 0.0, None, None, None)?;

        // Init best score
        let mut score = 0.;
        let mut depth_reached = 0;
        let mut node_count_reached = 0;

        // Iterative deepening
        for i_depth in 0..self.max_depth {
            if let Ok(dscore) = self.negamax(root, i_depth + 1, f32::NEG_INFINITY, f32::INFINITY) {
                score = dscore;
                depth_reached = i_depth;
                node_count_reached = self.pool.len();
            } else {
                break;
            }
        }

        // Extract principale variation
        let pv = self.extract_principal_variation(root);
        let max_qdepth = self.get_tree_max_depth(root);
        Some(SearchResult::new(
            pv,
            score,
            depth_reached.into(),
            max_qdepth,
            node_count_reached,
        ))
    }

    fn negamax(
        &mut self,
        node_handle: NodeHandle,
        depth: usize,
        mut alpha: f32,
        beta: f32,
    ) -> Result<f32, ()> {
        if depth == 0 {
            if self.is_tactical_node(node_handle) {
                return self.quiescence_search(node_handle, alpha, beta, 0);
            } else {
                let static_eval = self.pool.get_node(node_handle).ok_or(())?.get_score();

                // Set the best_score for this leaf node
                self.pool
                    .get_node_mut(node_handle)
                    .ok_or(())?
                    .set_best_score(static_eval);

                return Ok(static_eval);
            }
        }

        if !self
            .pool
            .get_node(node_handle)
            .expect("`negamax` need a valid node handle")
            .has_children_computed()
        {
            self.generate_children(node_handle)?;
        }

        let children = self.get_children_sorted_by_score(node_handle)?;

        if children.is_empty() {
            // Terminal position - return the static evaluation
            return Ok(self
                .pool
                .get_node(node_handle)
                .expect("`negamax` need a valid node handle")
                .get_score());
        }

        let mut best_score = f32::NEG_INFINITY;

        for child_handle in children {
            // Standard negamax recursion
            let score = -self.negamax(child_handle, depth - 1, -beta, -alpha)?;

            // Mate score adjustment
            let adjusted_score = if score.abs() > values::MATE_THRESHOLD {
                if score > 0.0 {
                    score - 1.0
                } else {
                    score + 1.0
                }
            } else {
                score
            };

            best_score = best_score.max(adjusted_score);
            alpha = alpha.max(adjusted_score);

            if alpha >= beta {
                break;
            }
        }

        // Store the best score for this node
        self.pool
            .get_node_mut(node_handle)
            .expect("`negamax` children need a valid node handle")
            .set_best_score(best_score);

        Ok(best_score)
    }

    fn quiescence_search(
        &mut self,
        node_handle: NodeHandle,
        mut alpha: f32,
        beta: f32,
        current_q_depth: usize,
    ) -> Result<f32, ()> {
        let stand_pat = self.pool.get_node(node_handle).ok_or(())?.get_score();

        if stand_pat >= beta {
            self.pool
                .get_node_mut(node_handle)
                .ok_or(())?
                .set_best_score(beta);
            return Ok(beta);
        }

        alpha = alpha.max(stand_pat);

        // Early return if we've reached max qsearch depth
        if current_q_depth >= self.max_q_depth {
            self.pool
                .get_node_mut(node_handle)
                .ok_or(())?
                .set_best_score(stand_pat);
            return Ok(stand_pat);
        }

        // Delta pruning
        if stand_pat + get_value_by_piece(Piece::Queen) < alpha {
            self.pool
                .get_node_mut(node_handle)
                .ok_or(())?
                .set_best_score(alpha);
            return Ok(alpha);
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

        let children = self.get_children_sorted_by_score(node_handle)?;
        let mut best_score = stand_pat;

        for child_handle in children {
            if self.is_tactical_node(child_handle) {
                //skip obvious bad captures at deeper levels
                if current_q_depth > 2 {
                    if let Some(child_node) = self.pool.get_node(child_handle) {
                        // Skip if this looks like a losing capture
                        if let Some(captured_piece) = child_node.get_captured_piece() {
                            let capture_value = get_value_by_piece(captured_piece);
                            if stand_pat + capture_value + 100.0 < alpha {
                                continue; // Skip this capture
                            }
                        }
                    }
                }

                let score =
                    -self.quiescence_search(child_handle, -beta, -alpha, current_q_depth + 1)?;

                best_score = best_score.max(score);
                alpha = alpha.max(score);

                if alpha >= beta {
                    break;
                }
            }
        }

        // IMPORTANT: Always set the best_score before returning
        self.pool
            .get_node_mut(node_handle)
            .ok_or(())?
            .set_best_score(best_score);
        Ok(best_score)
    }

    fn is_tactical_node(&self, handle: NodeHandle) -> bool {
        let node = self
            .pool
            .get_node(handle)
            .expect("`is_tactical_node` needs a valid handle");

        // Check if the move is a capture or gives check
        if node.get_engine().is_king_checked() {
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
        if node.get_engine().is_king_checked() {
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
                // let base_score = child.get_best_score().unwrap_or(child.get_score());
                let base_score = child.get_score();

                let player_move = child.get_move().ok_or(())?;
                let moved_piece = child.get_moved_piece();
                let captured_piece_opt = child.get_captured_piece();
                let is_king_checked = child.get_engine().is_king_checked();

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

    fn extract_principal_variation(&self, mut current_handle: NodeHandle) -> Vec<PlayerMove> {
        let mut pv = Vec::new();

        loop {
            let current_node = match self.pool.get_node(current_handle) {
                Some(node) => node,
                None => {
                    println!("PV: No node found for handle");
                    break;
                }
            };

            if current_node.get_children().is_empty() {
                // DEBUG: Check why this node has no children
                let engine = current_node.get_engine();
                let is_check = engine.is_king_checked();
                let has_children_computed = current_node.has_children_computed();

                println!("PV: No children, stopping at depth {}", pv.len());
                println!("PV: Node has_children_computed: {}", has_children_computed);
                println!("PV: King in check: {}", is_check);

                if has_children_computed {
                    println!("PV: This is a TERMINAL position (mate or stalemate)");
                    if is_check {
                        println!("PV: *** CHECKMATE FOUND! ***");
                    } else {
                        println!("PV: Stalemate position");
                    }
                } else {
                    println!("PV: Children not computed (search depth limit or leaf)");
                }

                // Also check the node's score for confirmation
                if let Some(best_score) = current_node.get_best_score() {
                    println!("PV: Final position score: {}", best_score);
                    if best_score.abs() > values::MATE_THRESHOLD {
                        println!("PV: *** MATE SCORE CONFIRMED! ***");
                    }
                }

                break;
            }

            println!(
                "PV: At depth {}, checking {} children",
                pv.len(),
                current_node.get_children().len()
            );

            let mut best_child_handle = None;
            let mut best_score = f32::NEG_INFINITY;
            let mut evaluated_children = 0;

            for &child_handle in current_node.get_children() {
                if let Some(child_node) = self.pool.get_node(child_handle) {
                    // Check if child has best_score
                    if let Some(child_best_score) = child_node.get_best_score() {
                        evaluated_children += 1;
                        let child_score = -child_best_score;

                        if child_score > best_score {
                            best_score = child_score;
                            best_child_handle = Some(child_handle);
                        }
                    } else {
                        // This child wasn't fully evaluated
                        println!("PV: Child without best_score found");
                    }
                }
            }

            println!(
                "PV: {}/{} children have best_score",
                evaluated_children,
                current_node.get_children().len()
            );

            match best_child_handle {
                Some(handle) => {
                    if let Some(best_child) = self.pool.get_node(handle) {
                        if let Some(move_) = best_child.get_move() {
                            pv.push(move_.clone());
                            println!("PV: Added move, now {} moves deep", pv.len());
                            current_handle = handle;
                        } else {
                            println!("PV: Best child has no move");
                            break;
                        }
                    } else {
                        println!("PV: Best child handle invalid");
                        break;
                    }
                }
                None => {
                    println!("PV: No best child found (no evaluated children)");
                    break;
                }
            }

            // Safety limit
            if pv.len() >= 20 {
                println!("PV: Hit safety limit");
                break;
            }
        }

        println!("PV: Final length: {}", pv.len());
        pv
    }

    fn get_tree_max_depth(&self, handle: NodeHandle) -> usize {
        if let Some(node) = self.pool.get_node(handle) {
            return node.get_children().iter().fold(0, |current_max, handle| {
                current_max.max(self.get_tree_max_depth(*handle))
            }) + 1;
        }
        0
    }
}
