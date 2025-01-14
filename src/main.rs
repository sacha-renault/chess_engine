pub mod boards;
pub mod game_engine;
pub mod pieces;
pub mod smart_engine;

// Define the prelude module
pub mod prelude {
    pub use super::game_engine::engine::Engine;
    pub use super::game_engine::move_results::{CorrectMoveResults, IncorrectMoveResults};
    pub use super::game_engine::player_move::{CastlingMove, NormalMove, PlayerMove};
    pub use super::game_engine::utility::{
        coordinates_to_u64, create_normal_move, iter_into_u64, u64_to_coordinates,
    };
}

use game_engine::debug::print_bitboard;
use prelude::{create_normal_move, iter_into_u64, Engine, NormalMove, PlayerMove};
use smart_engine::tree::{TreeNode, TreeBuilder, get_tree_size};
use smart_engine::evaluate::{Evaluator};
use smart_engine::values::get_value_by_piece;
use boards::Board;

struct Ev;
impl Evaluator for Ev {
    fn evaluate(&self, board: &Board) -> f32 {
        let mut score: f32 = 0.;
        for it in board.individual_pieces() {
            let piece = it.1;
            let color = it.2;
            let piece_score = get_value_by_piece(piece);
            score += piece_score * ((color as isize) as f32);
        }
        score
    }
}

fn main() {
    let engine = Engine::new();

    // White's turn: Move a pawn from e2 (1,4) to e4 (3,4)
    // let m = PlayerMove::Normal(NormalMove::new_from_coordinates((1, 4), (3, 4)));
    // let correct_move = engine.play(m); // White moves a pawn two squares forward

    let root = TreeNode::create_root_node(engine.clone());
    let tree_builder = TreeBuilder::new(Box::new(Ev { } ));
    tree_builder.generate_tree(root.clone(), 4);
    println!("Finished building tree");
    let result = get_tree_size(root.clone());
    println!("tree size is {}", result);

    for child in root.borrow().children() {
        let score = child.borrow().recursive_score();
        let m = child.borrow().chess_move().unwrap();
        println!("Score for move {:?} is {}" , m, score);
    }
}
