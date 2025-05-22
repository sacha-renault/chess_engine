use crate::pieces::Piece;
use crate::prelude::{Engine, PlayerMove};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeHandle(pub(super) usize);

pub struct TreeNode {
    // About the game
    engine: Engine,
    chess_move: Option<PlayerMove>,
    moved_piece: Option<Piece>,
    captured_piece: Option<Piece>,

    // About the tree
    score: f32,
    best_score: f32,
    computed: bool,
    children: Vec<NodeHandle>,
}

impl TreeNode {
    pub fn new(
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

    /// Returns a reference to the underlying chess engine
    pub fn get_engine(&self) -> &Engine {
        &self.engine
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
    pub fn get_children(&self) -> &Vec<NodeHandle> {
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

    /// Add a child to this node
    pub fn add_child(&mut self, child_handle: NodeHandle) {
        self.children.push(child_handle)
    }
}
