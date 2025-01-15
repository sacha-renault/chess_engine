use super::evaluate::Evaluator;
use crate::game_engine::get_move_row::GetMoveRow;
use crate::game_engine::player_move::PlayerMove;
use crate::game_engine::utility::get_color;
use crate::prelude::Engine;
use crate::pieces::Color;
use super::values;
use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<TreeNode>>;

#[derive(Clone)]
pub struct TreeNode {
    engine: Engine,
    children: Vec<NodeType>,
    raw_score: f32,
    chess_move: Option<PlayerMove>,
    computed: bool,
}

impl TreeNode {
    pub fn create_root_node(engine: Engine) -> NodeType {
        Rc::new(RefCell::new(TreeNode::new(engine, 0., None)))
    }

    pub fn new_cell(engine: Engine, score: f32, chess_move: Option<PlayerMove>) -> NodeType {
        Rc::new(RefCell::new(TreeNode::new(engine, score, chess_move)))
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    fn new(engine: Engine, score: f32, chess_move: Option<PlayerMove>) -> Self {
        // create the node
        let node = TreeNode {
            engine,
            children: Vec::new(),
            raw_score: score,
            chess_move,
            computed: false,
        };

        // use `set_recursive_score` to make the score flow to root
        node
    }

    pub fn score(&self) -> f32 {
        self.raw_score
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
            return self.raw_score;
        }

        // init a score
        let mut rec_score = 0.;

        // Add all scores from childrens
        for child in self.children.iter() {
            rec_score += child.borrow().recursive_score();
        }

        // weight by number of children
        self.raw_score + rec_score / num_children as f32
    }

    // Drop the refcount and therefore the entire branch is cleared
    pub fn drop_branch(&mut self) {
        self.children.clear();
    }
}

pub struct Tree {
    root: NodeType,
    evaluator: Box<dyn Evaluator>,
    max_depth: usize,
}

impl Tree {
    pub fn new(engine: Engine, evaluator: Box<dyn Evaluator>, max_depth: usize) -> Self {
        Tree {
            root: TreeNode::create_root_node(engine),
            evaluator,
            max_depth
        }
    }

    pub fn root(&self) -> NodeType {
        self.root.clone()
    }

    pub fn generate_tree(&self) {
        self.recursive_generate_tree(self.root.clone(), 0);
    }

    fn recursive_generate_tree(&self, node: NodeType, depth: usize) {
        // End tree building if reaching max depth
        if depth == self.max_depth {
            return;
        }

        // avoid recomputation
        if node.borrow_mut().computed {
            // Go deeper in the tree
            self.recursive_generate_tree(node, depth + 1);
        } else {
            // Get moves from this nodes
            let possible_moves = node
                .borrow()
                .engine
                .generate_moves_with_engine_state()
                .unwrap();

            // at this moment, we can se node to be computed
            node.borrow_mut().computed = true;

            // We check if the number of possible moves is 0
            if possible_moves.len() == 0 {
                // update the score (it might mean stale mate of checkmate)
                if node.borrow().engine.is_king_checked() {
                    let color_checkmate = get_color(!node.borrow().engine.white_to_play());
                    let multiplier: f32 = (color_checkmate as isize) as f32;
                    node.borrow_mut().raw_score = values::CHECK_MATE_VALUE * multiplier;
                } else {
                    // That's a draw
                    node.borrow_mut().raw_score = 0.;
                }
            } else {
                // iterate through all possible piece moving
                for move_row in possible_moves {
                    self.create_new_node(node.clone(), move_row, depth + 1);
                }
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
        self.recursive_generate_tree(child_node, depth);
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

    pub fn get_sorted_moves(&self) -> Vec<(PlayerMove, f32)> {
        // know who is it to play for this turn
        let white_to_play: bool = self.root.borrow().engine.white_to_play();

        // Collect all moves and their scores
        let mut moves: Vec<(PlayerMove, f32)> = self.root()
            .borrow()
            .children()
            .iter()
            .filter_map(|child| {
                let score = child.borrow().recursive_score();
                let m = child.borrow().chess_move().clone();
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

fn get_tree_size(root_node: NodeType) -> u64 {
    let node = root_node.borrow();
    let mut size = 1; // Count current node

    // Recursively count children
    for child in node.children.iter() {
        size += get_tree_size(child.clone());
    }

    size
}
