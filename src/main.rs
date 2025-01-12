mod simple_engine;

use simple_engine::engine::Engine;
use simple_engine::player_move::{NormalMove, PlayerMove};

fn main() {
    let mut engine = Engine::new();

    // White's turn: Move a pawn from e2 (1,4) to e4 (3,4)
    let m = PlayerMove::NormalMove(NormalMove::new_from_coordinates((1, 4), (2, 4)));
    let correct_move = engine.play(m); // White moves a pawn two squares forward

    match correct_move {
        Ok(_) => println!("Correct move!"),
        Err(string) => println!("Failed to make the move. {}", string),
    }

    // White's turn: Attempt to move a pawn from e2 (1,4) to e5 (4,4), which is not a legal move
    // let incorrect_move = engine.play((1, 4), (4, 4)); // White tries to move the pawn three squares forward, which is illegal

    // match incorrect_move {
    //     Ok(_) => println!("Correct move!"),
    //     Err(string) => println!("Failed to make the move.{}", string), // This should print since it's an invalid move
    // }
}
