use crate::prelude::{create_normal_move, iter_into_u64, CorrectMoveResults, Engine, NormalMove};
use crate::game_engine::player_move::PlayerMove;
use super::evaluate::Evaluator;
use crate::pieces::piece::PROMOTE_PIECE;
use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<TreeNode>>;

#[derive(Clone)]
pub struct TreeNode {
    engine: Engine,
    children: Vec<NodeType>,
    score: f32,
    chess_move: Option<PlayerMove>
}

impl TreeNode {
    pub fn create_root_node(engine: Engine) -> NodeType {
        Rc::new(RefCell::new(TreeNode::new(engine, 0., None)))
    }

    fn new(engine: Engine, score: f32, chess_move: Option<PlayerMove>) -> Self {
        // create the node
        let node = TreeNode {
            engine,
            children: Vec::new(),
            score,
            chess_move
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
        if  num_children == 0 {
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
}

pub struct TreeBuilder {
    evaluator: Box<dyn Evaluator>,
}

impl TreeBuilder {
    pub fn new(evaluator: Box<dyn Evaluator>) -> Self {
        TreeBuilder { evaluator }
    }

    pub fn generate_tree(&self, node: NodeType, depth: usize) {
        // End tree building if reaching max depth
        if depth == 0 {
            return;
        }

        // Clone the current engine
        let current_engine = node.borrow().engine.clone();

        // iterate through all possible piece moving
        for piece_moves in current_engine.get_all_moves_by_piece().unwrap() {
            // Case normal move
            match piece_moves.1 {
                PlayerMove::Normal(normal_move) => self.handle_normal_moves(node.clone(), current_engine.clone(), normal_move, depth),
                _ => { }
            }
        }
    }

    fn handle_normal_moves(&self, node: NodeType, engine: Engine, normal_move: NormalMove, depth: usize) {
        // Get target and current
        let (current_square, target_squares) = normal_move.squares();

        // iterate through all possible
        for possible_move in iter_into_u64(target_squares) {
            // Clone the current engine to play a move
            let mut new_engine = engine.clone();

            // Create the chess move
            let chess_move = create_normal_move(current_square, 1 << possible_move);

            // Move will always be correct (play_unsafe makes no checks)
            match new_engine.play(chess_move).unwrap() {
                CorrectMoveResults::Promote => {
                    // iterate over the possible promotion
                    for piece in PROMOTE_PIECE {
                        // create an engine for evey promotion piece
                        let mut promoting_new_engine = new_engine.clone();
                        let promotion_move = PlayerMove::Promotion(piece);
                        promoting_new_engine.play(promotion_move).unwrap();

                        // Create a new node for this possibility
                        self.create_new_node(node.clone(), promoting_new_engine, promotion_move, depth);
                    }
                }
                _ => {
                    // Create a new node for this possibility
                    self.create_new_node(node.clone(), new_engine, chess_move, depth);
                }
            }
        }
    }

    fn create_new_node(&self, node: NodeType, engine: Engine, chess_move: PlayerMove, depth: usize) {
        // Evaluate the board
        let score = self.evaluator.evaluate(&engine.board());

        // create a new node for the child
        let child_node = Rc::new(RefCell::new(TreeNode::new(engine, score, Some(chess_move))));

        // we add children into the node
        node.borrow_mut().children.push(child_node.clone());

        // We keep generating until depth reach 0
        self.generate_tree(child_node, depth - 1);
    }
}

pub fn get_tree_size(root_node: NodeType) -> u64 {
    let node = root_node.borrow();
    let mut size = 1; // Count current node

    // Recursively count children
    for child in node.children.iter() {
        size += get_tree_size(child.clone());
    }

    size
}
