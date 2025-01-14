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
        coordinates_to_u64, create_move_from_str, create_normal_move, iter_into_u64,
        string_from_move, u64_to_coordinates,
    };
}

use boards::Board;
use game_engine::debug::print_board;
use prelude::{
    create_move_from_str, iter_into_u64, string_from_move, Engine, NormalMove, PlayerMove,
};
use smart_engine::evaluate::Evaluator;
use smart_engine::tree::Tree;
use smart_engine::values::get_value_by_piece;

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
    let mut engine = Engine::new();

    let _ = engine.play(create_move_from_str("d2d4")).unwrap();
    let _ = engine.play(create_move_from_str("g8f6")).unwrap();
    let _ = engine.play(create_move_from_str("c1g5")).unwrap();
    let _ = engine.play(create_move_from_str("g7g6")).unwrap();
    let _ = engine.play(create_move_from_str("g5f6")).unwrap();
    let _ = engine.play(create_move_from_str("e7f6")).unwrap();
    let _ = engine.play(create_move_from_str("c2c3")).unwrap();
    let _ = engine.play(create_move_from_str("b7b5")).unwrap();
    let _ = engine.play(create_move_from_str("d1d3")).unwrap();
    let _ = engine.play(create_move_from_str("a7a5")).unwrap();
    let _ = engine.play(create_move_from_str("d3b5")).unwrap();
    let _ = engine.play(create_move_from_str("c8a6")).unwrap();
    let _ = engine.play(create_move_from_str("b5b8")).unwrap();
    let _ = engine.play(create_move_from_str("d8b8")).unwrap();
    let _ = engine.play(create_move_from_str("e1d1")).unwrap();
    let _ = engine.play(create_move_from_str("h7h6")).unwrap();
    let _ = engine.play(create_move_from_str("d1c2")).unwrap();
    let _ = engine.play(create_move_from_str("f8e7")).unwrap();
    let _ = engine.play(create_move_from_str("b1d2")).unwrap();
    let _ = engine.play(create_move_from_str("e7c5")).unwrap();
    let _ = engine.play(create_move_from_str("d4c5")).unwrap();
    let _ = engine.play(create_move_from_str("d7d6")).unwrap();
    let _ = engine.play(create_move_from_str("c5d6")).unwrap();
    let _ = engine.play(create_move_from_str("c7d6")).unwrap();
    let _ = engine.play(create_move_from_str("a1d1")).unwrap();
    let _ = engine.play(create_move_from_str("b8b5")).unwrap();
    let _ = engine.play(create_move_from_str("a2a4")).unwrap();
    let _ = engine.play(create_move_from_str("b5b6")).unwrap();
    let _ = engine.play(create_move_from_str("d2e4")).unwrap();
    let _ = engine.play(create_move_from_str("e8d7")).unwrap();
    let _ = engine.play(create_move_from_str("e4d6")).unwrap();
    let _ = engine.play(create_move_from_str("a8b8")).unwrap();
    let _ = engine.play(create_move_from_str("d6f7")).unwrap();
    let _ = engine.play(create_move_from_str("d7c8")).unwrap();
    let _ = engine.play(create_move_from_str("f7h8")).unwrap();
    let _ = engine.play(create_move_from_str("b6b2")).unwrap();

    print_board(engine.board());

    let tree = Tree::new(engine, Box::new(Ev {}));
    tree.generate_tree(4);
    let (bm, score) = tree.get_best_move();
    println!("{:?}", string_from_move(&bm));
    println!("score : {:?}", score);
}
