use std::cell::RefCell;
use std::rc::Rc;

use crate::game_engine::move_results::MoveResult;
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
    best_score: f32,
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
            best_score: 0.,
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
    
    /// Plays a move on this node's engine, modifying its position.
    /// 
    /// # Important Side Effects
    /// If the move is successfully played, this method clears ALL child nodes.
    /// This is necessary because playing a move directly (instead of selecting an existing branch)
    /// invalidates all previously calculated variations, as the new position might lead to
    /// completely different tactical possibilities.
    ///
    /// # Arguments
    /// * `mv` - The chess move to play
    ///
    /// # Returns
    /// * `MoveResult` - Success or failure of the move attempt
    ///
    /// # Implementation Note
    /// This differs from `select_branch()` which follows an existing calculated variation.
    /// Use `play()` when executing a new move that wasn't part of the calculated tree,
    /// and `select_branch()` when following a previously analyzed line.
    pub fn play(&mut self, mv: PlayerMove) -> MoveResult {
        let result = self.engine.play(mv);
        if result.is_ok() {
            self.children.clear();
        }
        return result;
    }

    /// Returns the raw evaluation score of this position
    pub fn get_score(&self) -> f32 {
        self.score
    }

    /// Get the best score
    pub fn get_best_score(&self) -> f32 {
        self.best_score
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

    /// Set the best score
    pub fn set_best_score(&mut self, score: f32) {
        self.best_score = score;
    }

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
    pub fn get_plies_to_mate(&self) -> Option<usize> {
        if self.best_score >= values::VALUE_TB_WIN_IN_MAX_PLY {
            // We need to solve: score = MATE_SCORE - (n * (n + 1) / 2)
            // Or: (n * (n + 1) / 2) = MATE_SCORE - score
            let diff = (values::CHECK_MATE - self.best_score.abs()) as f32;
            
            // Quadratic formula: n^2 + n - 2*diff = 0
            // (-1 ± sqrt(1 + 8*diff)) / 2
            let n = (-1.0 + f32::sqrt(1.0 + 8.0 * diff)) / 2.0;
            
            Some(n.ceil() as usize)
        } else {
            None
        }
    }

    /// Return the max depth reached for the current node
    pub fn get_depth(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.borrow().get_depth())
            .max()
            .unwrap_or(0) + 1
    }
}
