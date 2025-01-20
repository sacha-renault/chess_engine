use std::collections::HashSet;
use std::rc::Rc;

use crate::boards::zobrist_hash::Zobrist;
use crate::game_engine::move_evaluation_context::MoveEvaluationContext;
use crate::game_engine::player_move::PlayerMove;
use crate::game_engine::utility::get_color;
use crate::prelude::Engine;

use super::evaluate::Evaluator;
use super::node_with_score::NodeWithScore;
use super::transposition_table::TranspositionTable;
use super::tree_node::{TreeNode, TreeNodeRef};
use super::values;

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
    computation_allowed: bool,
}

impl Tree {
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
            computation_allowed: true,
        }
    }

    pub fn root(&self) -> TreeNodeRef {
        self.root.clone()
    }

    pub fn get_sorted_nodes(&self) -> Vec<TreeNodeRef> {
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

        children
    }

    pub fn generate_tree(&mut self) -> usize {
        // We start from depth - 1 (because last was select branch)
        self.current_depth = 1;

        // loop until one of break condition is matched
        loop {
            // brek condition (either too deep or size of the tree to big)
            if self.max_depth <= self.current_depth || self.size() > self.max_size {
                break;
            }

            // Start with worst possible values for alpha and beta
            let alpha = f32::NEG_INFINITY;
            let beta = f32::INFINITY;

            // Generate the tree recursively
            self.recursive_generate_tree(self.root.clone(), self.current_depth, alpha, beta);

            self.current_depth += 1;
        }

        self.current_depth
    }

    fn recursive_generate_tree(
        &mut self,
        node: TreeNodeRef,
        depth: usize,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        // End tree building if reaching max depth
        if depth == 0 {
            node.borrow_mut().set_raw_as_best();
            return node.borrow().get_raw_score();
        }

        // Avoid recomputation: Check if node is already computed
        let computed = node.borrow().is_computed();
        if !computed && self.computation_allowed {
            self.compute_new_children(node.clone());
        }

        // Get whos player is maximizing
        let is_maximizing = node.borrow().get_engine().white_to_play();
        let mut best_score = init_best_score(is_maximizing);
        // let scored_children = self.get_sorted_children_with_best_score(node.clone(), depth - 1);
        let scored_children = self.get_sorted_children_with_best_score(node.clone(), depth / 2);

        // TODO
        // Here we have to sort the child move (we can use the `recursive_generate_tree` function)
        for child in scored_children {
            // for child in node.borrow().get_children().iter() {
            let score = self.recursive_generate_tree(child.node(), depth - 1, alpha, beta);

            // Update the best score, alpha, and beta for pruning
            if is_maximizing {
                best_score = best_score.max(score);
                alpha = alpha.max(best_score);
            } else {
                best_score = best_score.min(score);
                beta = beta.min(best_score);
            }

            // Prune if the current branch can no longer affect the final result
            if beta <= alpha {
                break;
            }
        }

        // Set the best score of every node
        node.borrow_mut().set_best_score(best_score);
        best_score
    }

    fn compute_new_children(&mut self, node: TreeNodeRef) {
        // get the hash to see if this node exist somewhere in the tt
        let hash = self.hasher.compute_hash(
            node.borrow().get_engine().board(),
            node.borrow().get_engine().white_to_play(),
        );

        // // Check if the position is known in the transposition table
        if let Some(entry) = self.transpose_table.get_entry(hash) {
            // Only copy from completed nodes to avoid cycles
            node.replace_with(|_| entry.borrow().clone());
            return;
        }

        // everytime we create a children, we put it in our hashtable
        // to avoid recompute if we see it again
        self.transpose_table.insert_entry(hash, node.clone());

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
            let score = self.evaluator.evaluate(&possible_move.engine.board());

            // create a new node for the child
            let child_node =
                TreeNode::new_cell(possible_move.engine, score, Some(possible_move.player_move));

            // we add children into the node
            node.borrow_mut().add_child(child_node.clone());
        }
    }

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
                NodeWithScore::new(
                    child.clone(),
                    self.recursive_generate_tree(
                        child.clone(),
                        shallow_depth,
                        f32::NEG_INFINITY,
                        f32::INFINITY,
                    ),
                )
            })
            .collect::<Vec<NodeWithScore>>();

        if is_white_turn {
            scored_children.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap());
        } else {
            scored_children.sort_by(|a, b| a.score().partial_cmp(&b.score()).unwrap());
        }

        scored_children
    }

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
            score
        } else {
            node.borrow_mut().set_raw_score(0.);
            0.
        }
    }

    pub fn size(&self) -> usize {
        get_tree_size(self.root.clone())
    }

    pub fn select_branch(&mut self, chess_move: PlayerMove) -> Result<(), ()> {
        let kept_node = {
            // Find the node we want to keep
            if let Some(node) = self
                .root
                .borrow()
                .get_children()
                .iter()
                .find(|child| child.borrow().get_move() == &Some(chess_move))
            {
                // Clone the node we want to keep
                Some(node.clone())
            } else {
                None
            }
        };

        // Reassign root outside the borrowing scope
        if let Some(node) = kept_node {
            self.root = node;
            Ok(())
        } else {
            Err(())
        }
    }
}

fn get_tree_size(root_node: TreeNodeRef) -> usize {
    let mut visited = HashSet::new();
    get_tree_size_recursive(root_node, &mut visited)
}

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

fn init_best_score(is_maximizing: bool) -> f32 {
    if is_maximizing {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    }
}
