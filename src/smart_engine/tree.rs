use super::evaluate::Evaluator;
use crate::game_engine::get_move_row::GetMoveRow;
use crate::game_engine::player_move::PlayerMove;
use crate::prelude::{create_normal_move, iter_into_u64, CorrectMoveResults, Engine, NormalMove};
use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<TreeNode>>;

#[derive(Clone)]
pub struct TreeNode {
    engine: Engine,
    children: Vec<NodeType>,
    score: f32,
    chess_move: Option<PlayerMove>,
}

impl TreeNode {
    pub fn create_root_node(engine: Engine) -> NodeType {
        Rc::new(RefCell::new(TreeNode::new(engine, 0., None)))
    }

    pub fn new_cell(engine: Engine, score: f32, chess_move: Option<PlayerMove>) -> NodeType {
        Rc::new(RefCell::new(TreeNode::new(engine, score, chess_move)))
    }

    fn new(engine: Engine, score: f32, chess_move: Option<PlayerMove>) -> Self {
        // create the node
        let node = TreeNode {
            engine,
            children: Vec::new(),
            score,
            chess_move,
        };

        // use `set_recursive_score` to make the score flow to root
        node
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn children(&self) -> &Vec<NodeType> {
        &self.children
    }

    pub fn chess_move(&self) -> &Option<PlayerMove> {
        &self.chess_move
    }

    pub fn recursive_score(&self) -> f32 {
        // check if it has child, otherwise, it's just score value
        let num_children = self.children.len();
        if num_children == 0 {
            return self.score;
        }

        // init a score
        let mut rec_score = 0.;

        // Add all scores from childrens
        for child in self.children.iter() {
            rec_score += child.borrow().recursive_score();
        }

        // weight by number of children
        self.score + rec_score / num_children as f32
    }

    // Drop the refcount and therefore the entire branch is cleared
    pub fn drop_branch(&mut self) {
        self.children.clear();
    }
}

pub struct Tree {
    root: NodeType,
    evaluator: Box<dyn Evaluator>,
}

impl Tree {
    pub fn new(engine: Engine, evaluator: Box<dyn Evaluator>) -> Self {
        Tree {
            root: TreeNode::create_root_node(engine),
            evaluator,
        }
    }

    pub fn root(&self) -> NodeType {
        self.root.clone()
    }

    pub fn generate_tree(&self, depth: usize) {
        self.recursive_generate_tree(self.root.clone(), depth);
    }

    fn recursive_generate_tree(&self, node: NodeType, depth: usize) {
        // End tree building if reaching max depth
        if depth == 0 {
            return;
        }

        // check if children from this node already exists
        if node.borrow().children().len() != 0 {
            self.recursive_generate_tree(node, depth);
        } else {
            // Get moves from this nodes
            let possible_moves = node
                .borrow()
                .engine
                .generate_moves_with_engine_state()
                .unwrap();

            // iterate through all possible piece moving
            for move_row in possible_moves {
                self.create_new_node(node.clone(), move_row, depth);
            }
        }
    }

    fn create_new_node(&self, node: NodeType, move_row: GetMoveRow, depth: usize) {
        // Evaluate the board
        let score = self.evaluator.evaluate(&move_row.engine.board());

        // create a new node for the child
        let child_node = Rc::new(RefCell::new(TreeNode::new(
            move_row.engine,
            score,
            Some(move_row.player_move),
        )));

        // we add children into the node
        node.borrow_mut().children.push(child_node.clone());

        // We keep generating until depth reach 0
        self.recursive_generate_tree(child_node, depth - 1);
    }

    pub fn get_tree_size(&self) -> u64 {
        get_tree_size(self.root.clone())
    }

    pub fn select_branch(&mut self, chess_move: PlayerMove) -> bool {
        let kept_node = {
            // Find the node we want to keep
            if let Some(node) = self
                .root
                .borrow()
                .children
                .iter()
                .find(|child| child.borrow().chess_move == Some(chess_move))
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
            true
        } else {
            panic!("Unknown chess move ????");
        }
    }

    pub fn get_best_move(&self) -> (PlayerMove, f32) {
        let mut best_move = None;
        let mut best_score = f32::NEG_INFINITY;

        for child in self.root().borrow().children() {
            let score = child.borrow().recursive_score();
            let m = child.borrow().chess_move().unwrap();

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
        }

        (best_move.expect("No moves available"), best_score)
    }
}

fn get_tree_size(root_node: NodeType) -> u64 {
    let node = root_node.borrow();
    let mut size = 1; // Count current node

    // Recursively count children
    for child in node.children.iter() {
        size += get_tree_size(child.clone());
    }

    size
}
