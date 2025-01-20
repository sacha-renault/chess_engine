use crate::game_engine::player_move::PlayerMove;
use crate::prelude::Engine;
use std::cell::RefCell;
use std::rc::Rc;

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone)]
pub struct TreeNode {
    engine: Engine,
    children: Vec<TreeNodeRef>,
    raw_score: f32,
    chess_move: Option<PlayerMove>,
    computed: bool,
    best_score: f32,
}

impl TreeNode {
    // CREATE
    pub fn create_root_node(engine: Engine) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode::new(engine, 0., None)))
    }

    pub fn new_cell(engine: Engine, score: f32, chess_move: Option<PlayerMove>) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode::new(engine, score, chess_move)))
    }

    fn new(engine: Engine, score: f32, chess_move: Option<PlayerMove>) -> Self {
        // create the node
        let node = TreeNode {
            engine,
            children: Vec::new(),
            raw_score: score,
            chess_move,
            computed: false,
            best_score: 0.,
        };

        // use `set_recursive_score` to make the score flow to root
        node
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

    pub fn is_computed(&self) -> bool {
        self.computed
    }

    pub fn get_best_score(&self) -> f32 {
        self.best_score
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

    pub fn set_raw_as_best(&mut self) {
        self.best_score = self.raw_score;
    }

    fn recursive_check_mate_depth(&self, depth: isize) -> isize {
        if !self.computed {
            return -1;
        } else if self.children.len() == 0 {
            return depth;
        } else {
            for child in &self.children {
                let check_mate_depth = child.borrow().recursive_check_mate_depth(depth + 1);
                if check_mate_depth != -1 {
                    return check_mate_depth;
                }
            }
        }
        -1
    }

    pub fn check_mate_depth(&self) -> isize {
        return self.recursive_check_mate_depth(0);
    }

    // Drop the refcount and therefore the entire branch is cleared
    pub fn drop_branch(&mut self) {
        self.children.clear();
    }
}
