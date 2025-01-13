pub mod boards;
pub mod game_engine;
pub mod pieces;

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

fn main() {
    let engine = Engine::new();

    // White's turn: Move a pawn from e2 (1,4) to e4 (3,4)
    // let m = PlayerMove::Normal(NormalMove::new_from_coordinates((1, 4), (3, 4)));
    // let correct_move = engine.play(m); // White moves a pawn two squares forward

    for piece_moves in engine.get_all_moves_by_piece().unwrap() {
        for possible_move in iter_into_u64(piece_moves.2) {
            let mut new_engine = engine.clone();
            let m = create_normal_move(piece_moves.0, 1 << possible_move);
            new_engine.play(m).unwrap();
        }
    }
}
