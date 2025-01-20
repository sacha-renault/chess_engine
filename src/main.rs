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
use chess_engine::smart_engine::node_with_score;
use game_engine::debug::print_board;
use game_engine::player_move::PromotionMove;
use pieces::Piece;
use prelude::{
    create_move_from_str, iter_into_u64, string_from_move, Engine, NormalMove, PlayerMove,
};
use smart_engine::evaluate::Evaluator;
use smart_engine::node_with_score::NodeWithScore;
use smart_engine::tree::Tree;
use smart_engine::values::{get_value_by_piece, ValueRuleSet};
use std::panic;

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

fn play_robot_to_robot(depth: usize, size: usize) {
    let mut tree = Tree::new(Engine::new(), Box::new(ValueRuleSet {}), depth, size);
    let mut i = 0;

    while tree
        .root()
        .borrow()
        .get_engine()
        .get_all_moves_by_piece()
        .unwrap()
        .len()
        != 0
    {
        let depth_reached = tree.generate_tree();
        let scored_nodes = tree.get_sorted_nodes();
        let best_node = &scored_nodes[0];

        let played_str = {
            if tree.root().borrow().get_engine().white_to_play() {
                "White"
            } else {
                "Black"
            }
        };

        let best_move = best_node.borrow().get_move().unwrap();
        let _ = tree.select_branch(best_move.clone());
        println!("Tree size : {}", tree.size());

        // display the board
        println!(
            "{} - {} played: {} with score : {}",
            played_str,
            (tree.root().borrow().get_engine().halfmove_clock() + 1) / 2,
            string_from_move(&best_move),
            best_node.borrow().get_best_score()
        );
        println!("Number of moves available {}", scored_nodes.len());
        // println!("See mate ? {}", tree.root().borrow().check_mate_depth());
        for scored_node in scored_nodes.iter().skip(1).take(3) {
            println!(
                "     - also possible: {} with score: {}",
                string_from_move(&scored_node.borrow().get_move().unwrap()),
                scored_node.borrow().get_best_score()
            );
        }
        i += 1;
        print_board(tree.root().borrow().get_engine().board());
        // input!(String, "next ?");
        // if i > 0 {
        //     break;
        // }
    }
}

fn play_against_robot(is_white: bool, depth: usize, size: usize) {
    let mut engine = Engine::new();

    if is_white {
        // we exepct an input for first move
        let pm = input!(String, "Input a move: ");
        engine.play(create_move_from_str(&pm)).unwrap();
    }

    // Create the tree from the engine
    let mut tree = Tree::new(engine, Box::new(ValueRuleSet::new()), depth, size);

    loop {
        // Then the computer plays
        let depth_reached = tree.generate_tree();
        let nodes = tree.get_sorted_nodes();
        if nodes.len() == 0 {
            break;
        }

        let played_str = {
            if tree.root().borrow().get_engine().white_to_play() {
                "White"
            } else {
                "Black"
            }
        };

        let best_node = &nodes[0];
        // for (pm, score) in &moves {
        //     println!(
        //         "Best moves: {} with score : {}",
        //         string_from_move(&pm),
        //         score
        //     );
        // }

        let tree_size = tree.size();
        let _ = tree.select_branch(best_node.borrow().get_move().unwrap());
        print_board(tree.root().borrow().get_engine().board());
        println!(
            "{} played: {} with score {}. Tree size is : {}",
            played_str,
            string_from_move(&best_node.borrow().get_move().unwrap()),
            best_node.borrow().get_best_score(),
            tree_size
        );
        println!("Number of moves available : {}", nodes.len());

        for (index, node) in nodes.iter().enumerate().skip(1) {
            println!(
                "{} - Move : {:?} with score : {}",
                index,
                string_from_move(&node.borrow().get_move().unwrap()),
                node.borrow().get_best_score()
            )
        }
        tree.get_sorted_nodes();

        // user input
        let mut incorrect_move = true;
        while incorrect_move {
            let pm = input!(String, "Input a move: ");
            if pm == "moves".to_string() {
                let moves = tree.get_sorted_nodes();
                println!("Incorrect move, please retry : {}", moves.len());
                continue;
            }
            match tree.select_branch(create_move_from_str(&pm)) {
                Ok(()) => {
                    incorrect_move = false;
                }
                Err(()) => {
                    let moves = tree.get_sorted_nodes();
                    println!("Incorrect move, please retry : {}", moves.len());
                }
            }
        }
    }
}

fn main() {
    // test_promotion();
    play_against_robot(false, 20, 5e6 as usize);
    // play_against_robot(false, 5);
    // let ev = ValueRuleSet {};
    // let e = Engine::new();
    // let r = ev.evaluate(&e.board());
    // println!("{}", r);
    // println!("Base score : {}", tree.root().borrow().recursive_score());
    // for (pm, score ) in tree.get_sorted_moves() {
    //     println!("{} with score : {}", string_from_move(&pm), score);
    // }
}
