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

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};

use crate::boards::zobrist_hash::Zobrist;
use crate::game_engine::player_move::PlayerMove;
use crate::game_engine::utility::get_color;
use crate::prelude::Engine;

use super::evaluate::Evaluator;
use super::node_with_score::NodeWithScore;
use super::transposition_table::{TranspositionTable, TTFlag};
use super::tree_node::{TreeNode, TreeNodeRef};
use super::values;

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
    ) -> Self {
        Tree {
            root: TreeNode::create_root_node(engine),
            evaluator,
            max_depth,
            max_size,
            current_depth: 1,
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

    /// Returns children nodes sorted by their evaluation scores
    ///
    /// # Returns
    /// Vector of weak references to child nodes, sorted by score (descending for white, ascending for black)
    pub fn get_sorted_nodes(&self) -> Vec<Weak<RefCell<TreeNode>>> {
        let mut children = self.root.borrow().get_children().clone();

        // Use stored best_score instead of recomputing
        if self.root.borrow().get_engine().white_to_play() {
            children.sort_by(|a, b| {
                b.borrow()
                    .get_best_score()
                    .partial_cmp(&a.borrow().get_best_score())
                    .unwrap()
            });
        } else {
            children.sort_by(|a, b| {
                a.borrow()
                    .get_best_score()
                    .partial_cmp(&b.borrow().get_best_score())
                    .unwrap()
            });
        }

        children.iter().map(|child| Rc::downgrade(&child)).collect()
    }

    /// Generates the game tree using iterative deepening and alpha-beta pruning
    ///
    /// # Returns
    /// The maximum depth reached during tree generation
    pub fn generate_tree(&mut self) -> usize {
        // We start from depth - 1 (because last was select branch)
        self.current_depth = 1;

        // loop until one of break condition is matched
        loop {
            // Entries are depth dependent so we have to clear it
            self.transpose_table.clear();

            // break condition (either too deep or size of the tree to big)
            if self.max_depth <= self.current_depth || self.size() > self.max_size {
                break;
            }

            // Start with worst possible values for alpha and beta
            let alpha = f32::NEG_INFINITY;
            let beta = f32::INFINITY;

            // Generate the tree recursively
            self.minimax(self.root.clone(), self.current_depth, alpha, beta);

            self.current_depth += 1;
        }

        self.current_depth
    }

    /// Evaluates the current game state using the minimax algorithm.
    ///
    /// # Parameters
    /// * `node` - The node representing the current game state.
    /// * `depth` - The depth to which the evaluation should proceed.
    ///
    /// # Returns
    /// The static evaluation score of the game state for the given depth.
    fn minimax_evaluate(
        &mut self,
        node: TreeNodeRef,
        depth: usize,
        mut alpha: f32,
        mut beta: f32,
        is_foreseeing: bool,
    ) -> f32 {
        let is_maximizing = node.borrow().get_engine().white_to_play();
        let mut best_score = init_best_score(is_maximizing);
        let scored_children = self.get_sorted_children_with_best_score(node.clone(), depth / 2);

        for child in scored_children {
            let score = if is_foreseeing {
                self.minimax(child.node(), depth - 1, alpha, beta)
            } else {
                self.minimax_foreseeing(child.node(), depth - 1, alpha, beta)
            };

            // Update the best score, alpha, and beta for pruning
            if is_maximizing {
                best_score = best_score.max(score);
                alpha = alpha.max(best_score);
            } else {
                best_score = best_score.min(score);
                beta = beta.min(best_score);
            }

            // Prune if the current branch can no longer affect the result
            if beta <= alpha {
                break;
            }
        }

        best_score
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
    fn minimax(
        &mut self,
        node: TreeNodeRef,
        depth: usize,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        // get the hash to see if this node exist somewhere in the tt
        let hash = self.compute_node_hash(&node);

        // End tree building if reaching max depth
        if depth == 0 {
            self.transpose_table.insert_entry(hash, node.clone(), depth, TTFlag::Exact);
            node.borrow_mut().set_raw_as_best();
            return node.borrow().get_raw_score();
        }

        // Check the transposition table for existing results
        if let Some(best_score) = self.handle_transposition_table(hash, node.clone(), depth, &mut alpha, &mut beta) {
            return best_score;
        }

        // Check if children were already computed and if there were not, compute them
        if !node.borrow().has_children_computed() {
            self.compute_new_children(node.clone());
        }

        // Perform minimax evaluation
        let best_score = self.minimax_evaluate(node.clone(), depth, alpha, beta, false);

        // Insert results into the transposition table
        self.store_in_transposition_table(hash, node.clone(), depth, best_score, alpha, beta);

        // Set the best score of every node
        node.borrow_mut().set_best_score(best_score);
        best_score
    }

    /// Generates a foreseeing game tree using a shallow minimax search.
    /// This function is used to precompute potential game states without
    /// affecting the transposition table.
    ///
    /// # Parameters
    /// * `node` - The node representing the current game state.
    /// * `depth` - The depth to which the foreseeing search should proceed.
    ///
    /// # Returns
    /// The best score found during the foreseeing process.
    fn minimax_foreseeing(
        &mut self,
        node: TreeNodeRef,
        depth: usize,
        alpha: f32,
        beta: f32,
    ) -> f32 {
        // End tree building if reaching max depth
        if depth == 0 {
            node.borrow_mut().set_raw_as_best();
            return node.borrow().get_raw_score();
        }

        // Compute children if not already computed
        if !node.borrow().has_children_computed() {
            self.compute_new_children(node.clone());
        }

        // Perform minimax evaluation without storing results
        self.minimax_evaluate(node.clone(), depth, alpha, beta, true)
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
            let score = self.evaluator.evaluate(possible_move.engine.get_board());

            // create a new node for the child
            let child_node =
                TreeNode::new_cell(possible_move.engine, score, Some(possible_move.player_move));

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
        shallow_depth: usize,
    ) -> Vec<NodeWithScore> {
        // Clone the vec of children
        // It clone a Vec of ptr so
        // Cost is pretty small
        let children = node.borrow().get_children().clone();
        let is_white_turn = node.borrow().get_engine().white_to_play();

        // Map best score to each children
        let mut scored_children = children
            .into_iter()
            .map(|child| {
                // Calc score as a mix of foreseing best move
                let score = self.minimax_foreseeing(
                    child.clone(),
                    shallow_depth,
                    f32::NEG_INFINITY,
                    f32::INFINITY);

                // init the node with score
                NodeWithScore::new(child.clone(), score)
            })
            .collect::<Vec<NodeWithScore>>();

        if is_white_turn {
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
        if node.borrow().get_engine().is_king_checked() {
            // get who's check mated
            let color_checkmate = get_color(white_to_play);

            // multiplier = 1 if white, -1 if black
            let multiplier: f32 = (color_checkmate as isize) as f32;
            let score = values::CHECK_MATE_VALUE * multiplier;
            node.borrow_mut().set_raw_score(score);
            node.borrow_mut().set_raw_as_best();
            score
        } else {
            node.borrow_mut().set_raw_score(0.);
            node.borrow_mut().set_raw_as_best();
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
    ) -> Option<f32> {
        if let Some(entry) = self.transpose_table.get_entry(hash, depth) {
            let strong_ref = entry.node.upgrade().unwrap();

            match entry.flag {
                TTFlag::Exact => return Some(strong_ref.borrow().get_best_score()),
                TTFlag::LowerBound => *alpha = alpha.max(strong_ref.borrow().get_best_score()),
                TTFlag::UperBound => *beta = beta.min(strong_ref.borrow().get_best_score()),
            }

            if !node.borrow().has_children_computed() {
                node.borrow_mut().copy_entry(strong_ref.clone());
            }

            if *alpha >= *beta {
                return Some(strong_ref.borrow().get_best_score());
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
            TTFlag::UperBound
        } else if best_score >= beta {
            TTFlag::LowerBound
        } else {
            TTFlag::Exact
        };

        self.transpose_table.insert_entry(hash, node, depth, flag);
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
        return 0;
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
