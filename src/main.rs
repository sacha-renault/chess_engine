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
use smart_engine::tree::TreeNode;

fn main() {
    let engine = Engine::new();

    // White's turn: Move a pawn from e2 (1,4) to e4 (3,4)
    // let m = PlayerMove::Normal(NormalMove::new_from_coordinates((1, 4), (3, 4)));
    // let correct_move = engine.play(m); // White moves a pawn two squares forward

    let root = TreeNode::create_root_node(engine.clone());
    TreeNode::generate_tree(root.clone(), 5);
    let result = TreeNode::get_tree_size(root);
    println!("tree size is {}", result);
}
