pub mod boards;
pub mod game_engine;
pub mod pieces;

// Define the prelude module
pub mod prelude {
    pub use super::game_engine::engine::Engine;
    pub use super::game_engine::move_results::{CorrectMoveResults, IncorrectMoveResults};
    pub use super::game_engine::player_move::{CastlingMove, NormalMove, PlayerMove};
    pub use super::game_engine::utility::{coordinates_to_u64, u64_to_coordinates};
}
use game_engine::debug::print_bitboard;
use prelude::{Engine, NormalMove, PlayerMove};

fn main() {
    let mut engine = Engine::new();

    // White's turn: Move a pawn from e2 (1,4) to e4 (3,4)
    let m = PlayerMove::Normal(NormalMove::new_from_coordinates((1, 4), (3, 4)));
    let correct_move = engine.play(m); // White moves a pawn two squares forward

    match correct_move {
        // Ok(_) => (),
        Ok(_) => print_bitboard(engine.board().white.en_passant),
        Err(string) => println!("Failed to make the move. {:?}", string),
    }

    // White's turn: Attempt to move a pawn from e2 (1,4) to e5 (4,4), which is not a legal move
    // let incorrect_move = engine.play((1, 4), (4, 4)); // White tries to move the pawn three squares forward, which is illegal

    // match incorrect_move {
    //     Ok(_) => println!("Correct move!"),
    //     Err(string) => println!("Failed to make the move.{}", string), // This should print since it's an invalid move
    // }
}
