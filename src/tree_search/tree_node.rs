use std::cell::RefCell;
use std::rc::Rc;

use crate::game_engine::player_move::PlayerMove;
use crate::pieces::Piece;
use crate::game_engine::engine::Engine;
use crate::static_evaluation::values;

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub struct TreeNode {
    // About the tree
    children: Vec<TreeNodeRef>,
    score: f32,
    computed: bool,

    // About the game
    engine: Engine,
    chess_move: Option<PlayerMove>,
    moved_piece: Option<Piece>,
    captured_piece: Option<Piece>,
}

impl TreeNode {
    // CREATE
    pub fn create_root_node(engine: Engine) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode::new(engine, 0., None, None, None)))
    }

    pub fn new_cell(
        engine: Engine,
        score: f32,
        chess_move: Option<PlayerMove>,
        moved_piece: Piece,
        captured_piece: Option<Piece>,
    ) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode::new(
            engine,
            score,
            chess_move,
            Some(moved_piece),
            captured_piece,
        )))
    }

    fn new(
        engine: Engine,
        score: f32,
        chess_move: Option<PlayerMove>,
        moved_piece: Option<Piece>,
        captured_piece: Option<Piece>,
    ) -> Self {
        // create the node
        TreeNode {
            engine,
            children: Vec::new(),
            score,
            chess_move,
            computed: false,
            moved_piece,
            captured_piece,
        }
    }

    // GETTER

    /// Returns a reference to the underlying chess engine
    pub fn get_engine(&self) -> &Engine {
        &self.engine
    }
    
    /// Return a mutable reference of the Negine
    pub fn get_engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }

    /// Returns the raw evaluation score of this position
    pub fn get_score(&self) -> f32 {
        self.score
    }

    /// Returns a reference to the vector of child nodes
    pub fn get_children(&self) -> &Vec<TreeNodeRef> {
        &self.children
    }

    /// Returns a reference to the optional chess move that led to this position
    pub fn get_move(&self) -> &Option<PlayerMove> {
        &self.chess_move
    }

    /// Returns whether children nodes have been computed for this position
    pub fn has_children_computed(&self) -> bool {
        self.computed
    }

    /// Returns the piece that was moved to reach this position (panics on root node)
    pub fn get_moved_piece(&self) -> Piece {
        // this can panic only for very first root node that is
        // NEVER calling this fn
        self.moved_piece.unwrap()
    }

    /// Returns the piece that was captured in this move, if any
    pub fn get_captured_piece(&self) -> Option<Piece> {
        self.captured_piece
    }

    // SETTER

    /// Sets whether this node's children have been computed
    pub fn set_computed(&mut self, is_computed: bool) {
        self.computed = is_computed;
    }

    /// Sets the raw evaluation score for this position
    pub fn set_score(&mut self, score: f32) {
        self.score = score;
    }

    /// Adds a child node to this position's children
    pub fn add_child(&mut self, child: TreeNodeRef) {
        self.children.push(child);
    }

    /// Copies children, computed status, and best score from another node
    pub fn copy_entry(&mut self, node: TreeNodeRef) {
        self.children = node.borrow().children.clone();
        self.computed = true;
    }

    /// Returns the number of moves until checkmate, if the position is a forced mate
    ///
    /// # Returns
    /// * `Some(depth)` - The number of moves to reach checkmate if a forced mate exists
    /// * `None` - If there is no forced mate in the position
    pub fn get_mate_depth(&self) -> Option<usize> {
        self.recursive_get_mate_depth(0)
    }

    /// Return the depths of the current node
    pub fn get_depth(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.borrow().get_depth())
            .max()
            .unwrap_or(0) + 1
    }

    /// Recursively calculates the depth of a forced mate sequence from the current node
    ///
    /// # Arguments
    /// * `depth` - Current depth in the mate calculation (number of moves from root)
    ///
    /// # Returns
    /// * `Some(depth)` - Number of moves to reach checkmate from this position
    /// * `None` - If there is no forced mate sequence from this position
    ///
    /// # Notes
    /// * For White (maximizing): Returns the shortest mate sequence if ANY move leads to mate
    /// * For Black (minimizing): Returns the longest mate sequence only if ALL moves lead to mate
    /// * A position is considered mate when raw_score equals CHECK_MATE constant
    fn recursive_get_mate_depth(&self, depth: usize) -> Option<usize> {
        // If current node is a mate, we return its depth
        if self.score.abs() == values::CHECK_MATE {
            return Some(depth);
        }

        // If current node has no children, it is a leaf node
        // and we return None
        if self.children.is_empty() {
            return None;
        }

        // Get who is it to maximize
        let is_maximizing = self.engine.white_to_play();

        if is_maximizing {
            // For maximizing player (White), ANY move leading to mate is sufficient
            self.children
                .iter()
                .filter_map(|child| child.borrow().recursive_get_mate_depth(depth + 1))
                .min()
        } else {
            // For minimizing player (Black), ALL moves must lead to mate
            // If any move escapes mate, return None
            let mate_depths: Vec<_> = self.children
                .iter()
                .filter_map(|child| child.borrow().recursive_get_mate_depth(depth + 1))
                .collect();

            if mate_depths.len() != self.children.len() {
                None // Some move escapes mate
            } else {
                mate_depths.into_iter().max() // All moves lead to mate, take the longest (worst case)
            }
        }
    }
}
