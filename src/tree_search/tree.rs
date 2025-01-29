//! Tree implementation for chess game tree search
//!
//! This module provides a tree-based implementation for chess move analysis using
//! alpha-beta pruning and iterative deepening. Key components include:
//!
//! - `Tree`: Main structure managing the game tree exploration
//! - Move evaluation with alpha-beta pruning
//! - Transposition table support
//! - Size-limited tree growth
//! - Move sorting for better pruning efficiency
//!
//! The tree maintains a current position and can generate future positions up to
//! a specified depth or size limit.

use std::collections::HashSet;
use std::rc::Rc;
use std::usize;

use crate::boards::zobrist_hash::Zobrist;
use crate::game_engine::player_move::PlayerMove;
use crate::game_engine::utility::get_color;
use crate::game_engine::engine::Engine;
use crate::static_evaluation::evaluator_trait::Evaluator;
use crate::static_evaluation::values;

use super::minimax_output::MinimaxOutput;
use super::node_with_score::NodeWithScore;
use super::search_type::SearchType;
use super::transposition_table::{TTFlag, TranspositionTable};
use super::tree_node::{TreeNode, TreeNodeRef};
use super::utility::is_unstable_position;

/// A tree structure for chess move analysis
///
/// Manages a game tree starting from a root position, growing it within specified
/// depth and size limits using alpha-beta pruning for move evaluation.
pub struct Tree {
    // In new function
    root: TreeNodeRef,
    evaluator: Box<dyn Evaluator>,
    max_depth: usize,
    max_size: usize,
    max_q_depth: usize,
    foreseeing_windowing: f32,
    razoring_margin_base: f32,
    razoring_depth: usize,

    // auto initialized
    current_depth: usize,
    hasher: Zobrist,
    transpose_table: TranspositionTable,
}

impl Tree {
    /// Creates a new game tree with specified parameters
    ///
    /// # Parameters
    /// * `engine` - Initial game state
    /// * `evaluator` - Strategy for evaluating board positions
    /// * `max_depth` - Maximum depth to explore in the tree
    /// * `max_size` - Maximum number of nodes allowed in the tree
    ///
    /// # Returns
    /// A new Tree instance initialized with the given parameters
    pub fn new(
        engine: Engine,
        evaluator: Box<dyn Evaluator>,
        max_depth: usize,
        max_size: usize,
        max_q_depth: usize,
        foreseeing_windowing: f32,
        razoring_margin_base: f32,
        razoring_depth: usize,
    ) -> Self {
        Tree {
            root: TreeNode::create_root_node(engine),
            evaluator,
            max_depth,
            max_size,
            max_q_depth,
            current_depth: 1,
            foreseeing_windowing,
            razoring_margin_base,
            razoring_depth,
            hasher: Zobrist::new(),
            transpose_table: TranspositionTable::new(),
        }
    }

    /// Returns a reference to the root node of the tree
    ///
    /// # Returns
    /// Cloned reference to the root node
    pub fn root(&self) -> TreeNodeRef {
        self.root.clone()
    }

    /// Generates the game tree using iterative deepening and alpha-beta pruning
    ///
    /// # Returns
    /// The maximum depth reached during tree generation
    pub fn iterative_deepening(&mut self) -> MinimaxOutput {       
        // If the tree is already too big, we shouldn't return a null result
        // So we use our minimax tree with NO computation
        if self.size() > self.max_size {
            println!("Trying to get a result ...");
            return self.minimax(
                self.root().clone(),
                0,
                f32::NEG_INFINITY,
                f32::INFINITY);
        }

        // When starting iterative deepening, we remove previous results
        self.transpose_table.maintenance();

        // We start from depth = 1 (because last was select branch)
        self.current_depth = 1;
        let mut output= MinimaxOutput::new(None, 0.);

        // loop until one of break condition is matched
        loop {
            // Mark all entries as 'old'
            self.transpose_table.new_search();

            // break condition (either too deep or size of the tree to big)
            if self.max_depth < self.current_depth || self.size() > self.max_size {
                break;
            }

            // Start with worst possible values for alpha and beta
            let alpha = f32::NEG_INFINITY;
            let beta = f32::INFINITY;

            // Generate the tree recursively with minimax
            output = self.minimax(self.root.clone(), 0, alpha, beta);

            self.current_depth += 1;
        }

        output
    }

    /// Evaluates the current game state using the minimax algorithm.
    ///
    /// # Parameters
    /// * `node` - The node representing the current game state.
    /// * `alpha` - The minimum score that the maximizing player is assured of
    /// * `beta` - The maximum score that the minimizing player is assured of
    /// * `search_type` - - The type of search to perform:
    ///   - `SearchType::Full(depth)` - Standard minimax search to specified depth
    ///   - `SearchType::Quiescence(qdepth)` - Tactical evaluation search
    ///
    /// # Returns
    /// The static evaluation score of the game state for the given depth.
    fn minimax_evaluate(
        &mut self,
        node: TreeNodeRef,
        scored_children: Vec<NodeWithScore>,
        mut alpha: f32,
        mut beta: f32,
        search_type: SearchType,
        depth: usize
    ) -> MinimaxOutput {
        let is_maximizing = node.borrow().get_engine().white_to_play();
        let mut best_score = if scored_children.is_empty() {
            node.borrow().get_score()
        } else {
            init_best_score(is_maximizing)
        };
        let mut best_node = None;

        for child in scored_children.iter() {
            let minimax_output = match search_type {
                SearchType::Full =>
                    self.minimax(child.node(), depth + 1, alpha, beta),
                SearchType::Quiescence =>
                    self.quiescence_search(child.node(), alpha, beta, depth + 1)

            };
            let score = minimax_output.get_score();

            // Update best move if we found a better score
            if is_maximizing {
                if score > best_score {
                    best_score = score;
                    best_node = Some(child.node());
                }
                alpha = alpha.max(best_score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_node = Some(child.node());
                }
                beta = beta.min(best_score);
            }

            // Prune if the current branch can no longer affect the result
            if beta <= alpha && score.abs() != values::CHECK_MATE {
                break;
            }
        }

        MinimaxOutput::new(best_node, best_score)
    }

    /// Recursively generates the game tree using alpha-beta pruning.
    /// This function is the core of the minimax algorithm.
    ///
    /// # Parameters
    /// * `node` - The current node being processed.
    /// * `depth` - The remaining depth to explore.
    /// * `alpha` - The best score for the maximizing player so far.
    /// * `beta` - The best score for the minimizing player so far.
    ///
    /// # Returns
    /// The best score found for the current node.
    fn minimax(&mut self, node: TreeNodeRef, depth: usize, mut alpha: f32, mut beta: f32) -> MinimaxOutput {
        // get the hash to see if this node exist somewhere in the tt
        let hash = self.compute_node_hash(&node);

        // End tree building if reaching max depth
        if depth == self.current_depth {
            // Instead of just evaluating, call quiescence search
            let quiescence_output = 
                self.quiescence_search(node.clone(), alpha, beta, 0);

            // Store the quiescence score in the transposition table
            self.transpose_table
                .insert_entry(hash, node.clone(), depth, TTFlag::Exact, quiescence_output.get_score());

            return quiescence_output;
        }

        // Check for razoring
        // Can early prune the less promising nodes
        let white_to_play = node.borrow().get_engine().white_to_play();
        // if let Some(qval) = self.is_razoring_candidate(node.clone(), depth, alpha, white_to_play) {
        //     return qval;
        // }
        if let Some(_) = self.is_razoring_candidate(node.clone(), depth, alpha, white_to_play) {
            panic!("Not implemented");
        }

        // Check the transposition table for existing results
        if let Some(minimax_output) =
            self.handle_transposition_table(hash, node.clone(), depth, &mut alpha, &mut beta)
        {
            return minimax_output;
        }

        // Check if children were already computed and if there were not, compute them
        if !node.borrow().has_children_computed() {
            self.compute_new_children(node.clone());
        }

        // Get scored children
        let scored_children =
            self.get_sorted_children_with_best_score(node.clone());

        // Perform minimax evaluation
        let minimax_output = self.minimax_evaluate(
            node.clone(),
            scored_children,
            alpha,
            beta,
            SearchType::Full,
            depth
        );

        // Insert results into the transposition table
        self.store_in_transposition_table(hash, node.clone(), depth, minimax_output.get_score(), alpha, beta);
        minimax_output
    }

    /// Performs a Quiescence Search to evaluate positions with tactical sequences
    /// until a "quiet" position is reached. This helps prevent the horizon effect
    /// by extending the search in volatile positions.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - Mutable reference to the search engine
    /// * `node` - Reference to the current tree node being evaluated
    /// * `alpha` - The minimum score that the maximizing player is assured of
    /// * `beta` - The maximum score that the minimizing player is assured of
    /// * `qdepth` - Current depth in the quiescence search
    ///
    /// # Returns
    ///
    /// * `f32` - The evaluated score for the position
    ///
    /// # Algorithm
    ///
    /// 1. Depth Control:
    ///    - Stops if maximum quiescence depth is reached
    ///    - Returns raw evaluation score at maximum depth
    ///
    /// 2. Position Generation:
    ///    - Computes child positions if not already done
    ///    - Filters for only "unstable" positions (captures, checks, etc.)
    ///
    /// 3. Stand-Pat:
    ///    - Uses the current position's score as a lower bound
    ///    - Implements beta cutoff for early pruning
    ///    - Updates alpha if current position is better than previous alpha
    ///
    /// 4. Position Evaluation:
    ///    - If unstable positions exist, continues search with minimax
    ///    - If no unstable positions, returns the stand-pat score
    fn quiescence_search(
        &mut self,
        node: TreeNodeRef,
        mut alpha: f32,
        beta: f32,
        qdepth: usize,
    ) -> MinimaxOutput {
        // we want to limit qdepth to a certain level
        if qdepth >= self.max_q_depth {
            return MinimaxOutput::new(None, node.borrow().get_score());
        }

        // Children computation
        let is_computed = node.borrow().has_children_computed();
        if !is_computed {
            self.compute_new_children(node.clone());
        }

        // evaluate the current position
        let raw_score = node.borrow().get_score();

        // beta cutoff: opponent is already too good
        if raw_score >= beta {
            return MinimaxOutput::new(None, raw_score);
        }

        // // update alpha
        if raw_score > alpha {
            alpha = raw_score;
        }

        // we continue for all the nodes that are unstable
        let child_nodes: Vec<NodeWithScore> = self
            .get_sorted_children_with_best_score(node.clone())
            .into_iter()
            .filter(|scored_move| is_unstable_position(scored_move.node()))
            .collect::<Vec<_>>();

        // use minimax evaluation if there is at least one child
        if !child_nodes.is_empty() {
            self.minimax_evaluate(
                node,
                child_nodes,
                alpha,
                beta,
                SearchType::Quiescence,
                qdepth + 1
            )
        } else {
            MinimaxOutput::new(None, raw_score)
        }
    }

    /// Computes and adds all possible child nodes for a given position
    ///
    /// # Parameters
    /// * `node` - Node for which to generate children
    ///
    /// # Note
    /// Also handles terminal positions (checkmate/stalemate)
    fn compute_new_children(&mut self, node: TreeNodeRef) {
        // at this moment, we can se node to be computed
        node.borrow_mut().set_computed(true);

        // Get possible moves of the children
        let possible_moves = node
            .borrow()
            .get_engine()
            .generate_moves_with_engine_state()
            .unwrap_or_default();

        // If not possible move this is an end leaf
        if possible_moves.len() == 0 {
            self.evaluate_terminal_node(node.clone());
            return;
        }

        // add all the moves into node that will be
        // children of current node
        for possible_move in possible_moves.into_iter() {
            // calc the raw score of this board
            let score = self.evaluator.evaluate_engine_state(&possible_move.engine, self.current_depth);

            // create a new node for the child
            let child_node = TreeNode::new_cell(
                possible_move.engine,
                score,
                Some(possible_move.player_move),
                possible_move.piece,
                possible_move.captured_piece,
            );

            // we add children into the node
            node.borrow_mut().add_child(child_node.clone());
        }
    }

    /// Returns sorted children nodes with their evaluation scores
    ///
    /// # Parameters
    /// * `node` - Parent node whose children to sort
    /// * `shallow_depth` - Depth for preliminary evaluation
    ///
    /// # Returns
    /// Vector of nodes paired with their scores, sorted by score
    fn get_sorted_children_with_best_score(
        &mut self,
        node: TreeNodeRef,
    ) -> Vec<NodeWithScore> {
        // Clone the vec of children
        // It clone a Vec of ptr so
        // Cost is pretty small
        let children = node.borrow().get_children().clone();
        let is_white_to_play = node.borrow().get_engine().white_to_play();

        // Map best score to each children
        let mut scored_children = children
            .into_iter()
            .map(|child| {
                // Calculate some the hash to know if we already have a score for this node
                // Old score aren't perfect but for sufficient for move ordering
                let hash = self.compute_node_hash(&child);
                let base_score = self.transpose_table.get_old_entry_score(hash)
                    .unwrap_or_else(|| {
                        child.borrow().get_score()
                    });

                // heuristic eval
                let child_ref = child.borrow();
                let player_move = child_ref.get_move().unwrap();
                let moved_piece = child_ref.get_moved_piece();
                let captured_piece_opt = child_ref.get_captured_piece();
                let is_king_checked = child_ref.get_engine().is_current_king_checked();
                let bonus = self.evaluator.evaluate_heuristic_move(player_move,
                    moved_piece,
                    captured_piece_opt,
                    is_king_checked) * values::HEURISTIC_WEIGHT;

                // init the node with score
                if is_white_to_play {
                    NodeWithScore::new(child.clone(), base_score + bonus)
                } else {
                    NodeWithScore::new(child.clone(), base_score - bonus)
                }
            })
            .collect::<Vec<NodeWithScore>>();

        if is_white_to_play {
            scored_children.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap());
        } else {
            scored_children.sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());
        }

        scored_children
    }

    /// Evaluates a terminal node (checkmate or stalemate)
    ///
    /// # Parameters
    /// * `node` - Terminal node to evaluate
    ///
    /// # Returns
    /// Score for the terminal position (0 for stalemate, CHECK_MATE_VALUE for checkmate)
    fn evaluate_terminal_node(&self, node: TreeNodeRef) -> f32 {
        let white_to_play = node.borrow().get_engine().white_to_play();

        // if it's terminal node (number of moves == 0)
        // it means it's either check mate or stale mate
        if node.borrow().get_engine().is_current_king_checked() {
            // get who's check mated
            let color_checkmate = get_color(!white_to_play);

            // multiplier = 1 if white, -1 if black
            let multiplier: f32 = (color_checkmate as isize) as f32;
            let score = values::CHECK_MATE * multiplier;
            node.borrow_mut().set_score(score);
            score
        } else {
            node.borrow_mut().set_score(0.);
            0.
        }
    }

    /// Computes the hash for a given node based on the board and whose turn it is.
    ///
    /// # Arguments
    /// * `node` - A reference to the node whose hash needs to be computed.
    ///
    /// # Returns
    /// A 64-bit unsigned integer representing the hash of the node.
    fn compute_node_hash(&self, node: &TreeNodeRef) -> u64 {
        self.hasher.compute_hash(
            node.borrow().get_engine().get_board(),
            node.borrow().get_engine().white_to_play(),
        )
    }

    /// Checks the transposition table for an existing entry and updates alpha and beta values accordingly.
    ///
    /// # Arguments
    /// * `hash` - A 64-bit hash value representing the current node.
    /// * `node` - A reference to the current node in the search tree.
    /// * `depth` - The current search depth.
    /// * `alpha` - A mutable reference to the alpha value used in alpha-beta pruning.
    /// * `beta` - A mutable reference to the beta value used in alpha-beta pruning.
    ///
    /// # Returns
    /// An optional f32 value representing the best score if a valid entry is found, otherwise `None`.
    fn handle_transposition_table(
        &mut self,
        hash: u64,
        node: TreeNodeRef,
        depth: usize,
        alpha: &mut f32,
        beta: &mut f32,
    ) -> Option<MinimaxOutput> {
        if let Some(entry) = self.transpose_table.get_entry(hash, depth) {
            // Upgrade to a strong ref
            let strong_ref = entry.node.upgrade().unwrap();
            let score = entry.score;

            // Build MinimaxOutput with the stored node
            let output = MinimaxOutput::new(
                None,
                score);

            // match flag to know what to do
            match entry.flag {
                TTFlag::Exact => return Some(output),
                TTFlag::LowerBound => *alpha = alpha.max(strong_ref.borrow().get_score()),
                TTFlag::UpperBound => *beta = beta.min(strong_ref.borrow().get_score()),
            }

            // Copy information that we care from the node
            // if !node.borrow().has_children_computed() {
            //     node.borrow_mut().copy_entry(strong_ref.clone());
            // }

            // Get raw score
            let raw_score = strong_ref.borrow().get_score();
            node.borrow_mut().set_score(raw_score);
            if *alpha >= *beta {
                return Some(output);
            }
        }
        None
    }

    /// Stores a new entry in the transposition table with the computed best score and flags.
    ///
    /// # Arguments
    /// * `hash` - A 64-bit hash value representing the current node.
    /// * `node` - A reference to the current node in the search tree.
    /// * `depth` - The current search depth.
    /// * `best_score` - The best score found for the current node.
    /// * `alpha` - The alpha value used in alpha-beta pruning.
    /// * `beta` - The beta value used in alpha-beta pruning.
    ///
    /// # Returns
    /// This function has no return value.
    fn store_in_transposition_table(
        &mut self,
        hash: u64,
        node: TreeNodeRef,
        depth: usize,
        best_score: f32,
        alpha: f32,
        beta: f32,
    ) {
        let flag = if best_score <= alpha {
            TTFlag::UpperBound
        } else if best_score >= beta {
            TTFlag::LowerBound
        } else {
            TTFlag::Exact
        };

        self.transpose_table.insert_entry(hash, node, depth, flag, best_score);
    }

    /// Checks if a node is a candidate for razoring based on the current depth and alpha value.
    fn is_razoring_candidate(&mut self,
        node: TreeNodeRef,
        depth: usize,
        alpha: f32,
        white_to_play: bool
    ) -> Option<MinimaxOutput> {
        // Avoid pruning branch too early
        if depth <= self.razoring_depth {
            return None;
        }

        // razoring threshold
        let razoring_threshold = self.razoring_margin_base *
            f32::powi(values::RAZORING_DEPTH_MULTIPLIER, (depth - self.razoring_depth) as i32);

        // Get the static evaluation of the node
        let score = node.borrow().get_score();

        // Should razor
        let should_razor = if white_to_play {
            score < razoring_threshold
        } else {
            score > -razoring_threshold
        };

        // If eval is below the threshold, perform a quiescence search
        if should_razor {
            let qval = self.quiescence_search(node.clone(), alpha - 1.0, alpha, 0);

            // Check if value fails low and is within a reasonable bound
            let score = qval.get_score();
            if score < alpha && score.abs() < values::VALUE_TB_WIN_IN_MAX_PLY {
                return Some(qval); // Fail low
            }
        }

        None
    }

    /// Calculates the total number of nodes in the tree
    ///
    /// # Returns
    /// Total count of nodes in the tree, avoiding double counting in cycles
    pub fn size(&self) -> usize {
        get_tree_size(self.root.clone())
    }

    /// Selects a branch of the tree by following the given move
    ///
    /// # Parameters
    /// * `chess_move` - Move to follow in the tree
    ///
    /// # Returns
    /// Ok(()) if move was found and selected, Err(()) if move wasn't found
    pub fn select_branch(&mut self, chess_move: PlayerMove) -> Result<(), ()> {
        let mut kept_node: Option<Rc<std::cell::RefCell<TreeNode>>> = None;

        for child in self.root.borrow().get_children() {
            if child.borrow().get_move() == &Some(chess_move) {
                kept_node = Some(child.clone());
                break;
            }
        }

        // Reassign root outside the borrowing scope
        if let Some(node) = kept_node {
            self.root = node;
            self.current_depth -= 1; // We virtual reduce the size of the tree
            Ok(())
        } else {
            Err(())
        }
    }
}

/// Calculates the total size of a tree starting from given root
///
/// # Parameters
/// * `root_node` - Starting node for size calculation
///
/// # Returns
/// Total number of unique nodes in the tree
fn get_tree_size(root_node: TreeNodeRef) -> usize {
    let mut visited = HashSet::new();
    get_tree_size_recursive(root_node, &mut visited)
}

/// Helper function for tree size calculation that handles cycles
///
/// # Parameters
/// * `root_node` - Current node being processed
/// * `visited` - Set of already visited nodes
///
/// # Returns
/// Number of unique nodes in this subtree
fn get_tree_size_recursive(
    root_node: TreeNodeRef,
    visited: &mut HashSet<*const std::cell::RefCell<TreeNode>>,
) -> usize {
    let raw_ptr = Rc::as_ptr(&root_node);

    // If this node has already been visited, return 0 to prevent double counting
    if !visited.insert(raw_ptr) {
        panic!("CYCLIC REFERENCES !!??");
    }

    let node = root_node.borrow();
    let mut size = 1; // Count current node

    // Recursively count children
    for child in node.get_children().iter() {
        size += get_tree_size_recursive(child.clone(), visited);
    }

    size
}

/// Returns initial score based on whether the player is maximizing
///
/// # Parameters
/// * `is_maximizing` - Whether the current player is maximizing
///
/// # Returns
/// Negative infinity for maximizing player, positive infinity for minimizing player
fn init_best_score(is_maximizing: bool) -> f32 {
    if is_maximizing {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    }
}
