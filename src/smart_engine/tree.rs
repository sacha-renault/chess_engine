use super::evaluate::Evaluator;
use super::tree_node::{TreeNode, TreeNodeRef};
use super::values;
use crate::game_engine::get_move_row::GetMoveRow;
use crate::game_engine::player_move::PlayerMove;
use crate::game_engine::utility::get_color;
use crate::prelude::Engine;
use std::cmp;

pub struct Tree {
    root: TreeNodeRef,
    evaluator: Box<dyn Evaluator>,
    max_depth: usize,
}

impl Tree {
    pub fn new(engine: Engine, evaluator: Box<dyn Evaluator>, max_depth: usize) -> Self {
        Tree {
            root: TreeNode::create_root_node(engine),
            evaluator,
            max_depth,
        }
    }

    pub fn root(&self) -> TreeNodeRef {
        self.root.clone()
    }

    pub fn generate_tree(&self) {
        // Start with worst possible values for alpha-beta
        let alpha = f32::NEG_INFINITY;
        let beta = f32::INFINITY;
        self.recursive_generate_tree(self.root.clone(), 0, alpha, beta);
    }

    fn recursive_generate_tree(
        &self,
        node: TreeNodeRef,
        depth: usize,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        // End tree building if reaching max depth
        if depth == self.max_depth {
            return node.borrow().get_score();
        }

        // avoid recomputation
        let computed = node.borrow().is_computed();
        if computed {
            // get whos to maximize (black or white)
            let is_maximizing = node.borrow().get_engine().white_to_play();
            let mut best_score = init_best_score(is_maximizing);

            for child in node.borrow().get_children().iter() {
                // Go deeper in the tree for every child
                let score = self.recursive_generate_tree(child.clone(), depth + 1, alpha, beta);

                // update the best score, alpha and beta for pruning
                if is_maximizing {
                    best_score = best_score.max(score);
                    alpha = alpha.max(best_score);
                } else {
                    best_score = best_score.min(score);
                    beta = beta.min(best_score);
                }

                if beta <= alpha {
                    break;
                }
            }
            return best_score;
        } else {
            return self.process_node(node.clone(), depth, alpha, beta);
        }
    }

    fn process_node(&self, node: TreeNodeRef, depth: usize, mut alpha: f32, mut beta: f32) -> f32 {
        // get whos to maximize (black or white)
        let is_maximizing = node.borrow().get_engine().white_to_play();

        // Get moves from this nodes
        let possible_moves = node
            .borrow()
            .get_engine()
            .generate_moves_with_engine_state()
            .unwrap();
        let mut scored_moves: Vec<(GetMoveRow, f32)> = possible_moves
            .into_iter()
            .map(|move_row| {
                let score = self.evaluator.evaluate(&move_row.engine.board());
                (move_row, score)
            })
            .collect();

        // Sort moves based on the scores
        if is_maximizing {
            scored_moves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        } else {
            scored_moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        }

        // at this moment, we can se node to be computed
        node.borrow_mut().set_computed(true);

        // We check if the number of possible moves is 0
        if scored_moves.len() == 0 {
            // update the score (it might mean stale mate of checkmate)
            if node.borrow().get_engine().is_king_checked() {
                let color_checkmate = get_color(!node.borrow().get_engine().white_to_play());
                let multiplier: f32 = (color_checkmate as isize) as f32;
                let score = values::CHECK_MATE_VALUE * multiplier;
                node.borrow_mut().set_score(score);
                return score;
            } else {
                // That's a draw
                node.borrow_mut().set_score(0.);
                return 0.;
            }
        } else {
            // init a best score for
            let mut best_score = init_best_score(is_maximizing);

            // iterate through all possible piece moving
            for (move_row, _) in scored_moves {
                // create the new node and get the score for it
                let score = self.create_new_node(node.clone(), move_row, depth + 1, alpha, beta);

                // update the best score, alpha and beta for pruning
                if is_maximizing {
                    best_score = best_score.max(score);
                    alpha = alpha.max(best_score);
                } else {
                    best_score = best_score.min(score);
                    beta = beta.min(best_score);
                }

                if beta <= alpha {
                    break; // Stop exploring this branch
                }
            }

            node.borrow_mut().set_score(best_score);
            best_score
        }
    }

    fn create_new_node(
        &self,
        node: TreeNodeRef,
        move_row: GetMoveRow,
        depth: usize,
        alpha: f32,
        beta: f32,
    ) -> f32 {
        // Evaluate the board
        let score = self.evaluator.evaluate(&move_row.engine.board());

        // create a new node for the child
        let child_node = TreeNode::new_cell(move_row.engine, score, Some(move_row.player_move));

        // we add children into the node
        node.borrow_mut().add_child(child_node.clone());

        // We keep generating until depth reach 0
        return self.recursive_generate_tree(child_node, depth, alpha, beta);
    }

    pub fn size(&self) -> u64 {
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

    pub fn get_sorted_moves(&self) -> Vec<(PlayerMove, f32)> {
        // know who is it to play for this turn
        let white_to_play: bool = self.root.borrow().get_engine().white_to_play();

        // Collect all moves and their scores
        let mut moves: Vec<(PlayerMove, f32)> = self
            .root()
            .borrow()
            .get_children()
            .iter()
            .filter_map(|child| {
                let score = child.borrow().get_score();
                let m = child.borrow().get_move().clone();
                m.map(|mv| (mv, score))
            })
            .collect();

        // Sort moves based on the player
        if white_to_play {
            moves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        } else {
            moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        }

        moves
    }
}

fn get_tree_size(root_node: TreeNodeRef) -> u64 {
    let node = root_node.borrow();
    let mut size = 1; // Count current node

    // Recursively count children
    for child in node.get_children().iter() {
        size += get_tree_size(child.clone());
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
