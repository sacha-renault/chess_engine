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
use chess_engine::game_engine::engine;
use game_engine::debug::print_board;
use game_engine::player_move::PromotionMove;
use pieces::Piece;
use prelude::{
    create_move_from_str, iter_into_u64, string_from_move, Engine, NormalMove, PlayerMove,
};
use smart_engine::evaluate::Evaluator;
use smart_engine::tree::Tree;
use smart_engine::values::get_value_by_piece;


use std::io::Write;

macro_rules! input {
    ($t:ty) => {{
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        let a: $t = a.trim().parse().unwrap();
        a
    }};
    (String) => {{
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        a.trim().to_string()
    }};
    ($t:ty, $txt:expr) => {{
        print!("{}", $txt);
        std::io::stdout().flush().unwrap();
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        let a: $t = a.trim().parse().unwrap();
        a
    }};
    (String, $txt:expr) => {{
        print!("{}", $txt);
        std::io::stdout().flush().unwrap();
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        a.trim().to_string()
    }};
}

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

fn play_robot_to_robot() {
    let mut tree = Tree::new(Engine::new(), Box::new(Ev {}), 4);

    while tree.root().borrow().engine().get_all_moves_by_piece().unwrap().len() != 0 {
        tree.generate_tree();
        let moves = tree.get_sorted_moves();
        let best_move = moves[0];
        tree.select_branch(best_move.0);
        print_board(tree.root().borrow().engine().board());
    }
}

fn play_against_computer(is_white: bool) {
    let mut engine = Engine::new();

    if is_white {
        // we exepct an input for first move
        let pm = input!(String, "Input a move: ");
        engine.play(create_move_from_str(&pm)).unwrap();
    }

    // Create the tree from the engine
    let mut tree = Tree::new(engine, Box::new(Ev {}), 4);

    loop {
        // Then the computer plays
        tree.generate_tree();
        let moves = tree.get_sorted_moves();
        if moves.len() == 0 {
            break;
        }
        let best_move = moves[0];
        println!("Computer played: {}", string_from_move(&best_move.0));
        tree.select_branch(best_move.0);

        // we exepct an input for first move
        let pm = input!(String, "Input a move: ");
        tree.select_branch(create_move_from_str(&pm));
    }
}

fn test_promotion() {
    let mut engine = Engine::new();
    let _ = engine.play(create_move_from_str("a2a4"));
    let _ = engine.play(create_move_from_str("b7b5"));
    let _ = engine.play(create_move_from_str("a4b5"));
    let _ = engine.play(create_move_from_str("h7h6"));
    let _ = engine.play(create_move_from_str("b5b6"));
    let _ = engine.play(create_move_from_str("h6h5"));
    let _ = engine.play(create_move_from_str("b6b7"));
    let _ = engine.play(create_move_from_str("h5h4"));
    let _ = engine.play(create_move_from_str("b7a8")).unwrap();
    let mv: PlayerMove = create_move_from_str("b7a8");

    match mv {
        PlayerMove::Normal(nmv) => {
            let (c, t) = nmv.squares();
            let propomotion_move = PlayerMove::Promotion(PromotionMove::new(c, t, Piece::Queen));
            let result = engine.play(propomotion_move).unwrap();
            println!("{:?}", result);
            print_board(engine.board());
        }
        _ => { }
    }
}
fn main() {
    // test_promotion();
    // play_robot_to_robot();
    play_against_computer(true);
}
