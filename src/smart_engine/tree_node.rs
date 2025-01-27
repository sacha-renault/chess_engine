use crate::game_engine::player_move::PlayerMove;
use crate::pieces::Piece;
use crate::prelude::Engine;
use std::cell::RefCell;
use std::rc::Rc;

use super::values;

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub struct TreeNode {
    // About the tree
    children: Vec<TreeNodeRef>,
    raw_score: f32,
    computed: bool,
    best_score: f32,

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
            raw_score: score,
            chess_move,
            computed: false,
            best_score: 0.,
            moved_piece,
            captured_piece,
        }
    }

    // GETTER
    pub fn get_engine(&self) -> &Engine {
        &self.engine
    }

    pub fn get_raw_score(&self) -> f32 {
        self.raw_score
    }

    pub fn get_children(&self) -> &Vec<TreeNodeRef> {
        &self.children
    }

    pub fn get_move(&self) -> &Option<PlayerMove> {
        &self.chess_move
    }

    pub fn has_children_computed(&self) -> bool {
        self.computed
    }

    pub fn get_best_score(&self) -> f32 {
        self.best_score
    }

    pub fn get_moved_piece(&self) -> Piece {
        // this can panic only for very first root node that is
        // NEVER calling this fn
        self.moved_piece.unwrap()
    }

    pub fn get_captured_piece(&self) -> Option<Piece> {
        self.captured_piece
    }

    // SETTER
    pub fn set_computed(&mut self, is_computed: bool) {
        self.computed = is_computed;
    }

    pub fn set_raw_score(&mut self, score: f32) {
        self.raw_score = score;
    }

    pub fn add_child(&mut self, child: TreeNodeRef) {
        self.children.push(child);
    }

    pub fn set_best_score(&mut self, score: f32) {
        self.best_score = score;
    }

    pub fn copy_entry(&mut self, node: TreeNodeRef) {
        self.children = node.borrow().children.clone();
        self.computed = true;
        self.best_score = node.borrow().best_score;
    }

    pub fn get_mate_depth(&self) -> Option<usize> {
        if self.best_score.abs() == values::CHECK_MATE {
            self.recursive_get_mate_depth(0)
        } else {
            None
        }
    }

    fn recursive_get_mate_depth(&self, depth: usize) -> Option<usize> {
        if self.raw_score.abs() == values::CHECK_MATE {
            Some(depth)
        } else if self.children.is_empty() {
            return None;
        } else {
            self.children
                .iter()
                .filter_map(|child| child.borrow().recursive_get_mate_depth(depth + 1))
                .min()
        }
    }
}
