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
use core::f32;
use game_engine::debug::print_board;
use game_engine::player_move::PromotionMove;
use game_engine::utility::move_piece;
use pieces::Piece;
use prelude::{
    create_move_from_str, iter_into_u64, string_from_move, Engine, NormalMove, PlayerMove,
};
use smart_engine::evaluate::Evaluator;
use smart_engine::node_with_score::NodeWithScore;
use smart_engine::tree::Tree;
use smart_engine::tree_builder::TreeBuilder;
use smart_engine::tree_node::TreeNode;
use smart_engine::values::{get_value_by_piece, ValueRuleSet};
use std::cell::RefCell;
use std::panic;

use std::io::Write;
use std::rc::{Rc, Weak};

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

// fn play_robot_to_robot(depth: usize, size: usize, display: bool) {
//     let mut engine = Engine::new();
//     engine.play(create_move_from_str("e2e4")).unwrap();
//     engine.play(create_move_from_str("e7e5")).unwrap();
//     engine.play(create_move_from_str("f1e2")).unwrap();
//     engine.play(create_move_from_str("f8e7")).unwrap();
//     engine.play(create_move_from_str("g1f3")).unwrap();
//     engine.play(create_move_from_str("g8f6")).unwrap();
//     // print_board(engine.get_board());

//     let mut tree = Tree::new(engine, Box::new(ValueRuleSet {}), depth, size);
//     let mut i = 0;

//     while tree
//         .root()
//         .borrow()
//         .get_engine()
//         .get_all_moves_by_piece()
//         .len()
//         != 0
//     {
//         let depth_reached = tree.generate_tree();
//         let scored_nodes = tree.get_sorted_nodes();
//         let best_node = &scored_nodes[0];
//         let best_move = best_node.upgrade().unwrap().borrow().get_move().unwrap();

//         let played_str = {
//             if tree.root().borrow().get_engine().white_to_play() {
//                 "White"
//             } else {
//                 "Black"
//             }
//         };

//         let tree_size_before_select = tree.size();

//         // display the board
//         match best_node.upgrade() {
//             Some(mv) => println!(
//                 "{} - {} played: {} with score : {} (depth = {} & tree size : {})",
//                 played_str,
//                 (tree.root().borrow().get_engine().get_halfmove_clock() + 1) / 2,
//                 string_from_move(&best_move),
//                 mv.borrow().get_best_score(),
//                 depth_reached,
//                 tree_size_before_select
//             ),
//             None => println!("What the fuck ? no moves ?"),
//         }

//         for scored_node in scored_nodes.iter().skip(1).take(3) {
//             println!(
//                 "     - also possible: {} with score: {}",
//                 string_from_move(&scored_node.upgrade().unwrap().borrow().get_move().unwrap()),
//                 scored_node.upgrade().unwrap().borrow().get_best_score()
//             );
//         }
//         let _ = tree.select_branch(best_move.clone());

//         i += 1;
//         if display {
//             print_board(tree.root().borrow().get_engine().get_board());
//         }

//         // if i > 3 {
//         //     break;
//         // }
//     }
// }

fn play_against_robot(is_white: bool, depth: usize, size: usize) {
    let mut engine = Engine::new();
    let _ = engine.play(create_move_from_str("e2e4")); // Force an initial move of e4

    if !is_white {
        // we exepct an input for first move
        let pm = input!(String, "Input a move: ");
        let mv = engine.get_move_by_str(&pm).unwrap();
        engine.play(mv).unwrap();
    }

    // Create the tree from the engine
    let mut tree = TreeBuilder::new()
        .max_depth(depth)
        .max_size(size)
        .foreseeing_windowing(f32::INFINITY)
        .build_tree(engine, Box::new(ValueRuleSet::new()))
        .unwrap();

    let root_ref = Rc::downgrade(&tree.root());
    println!("Root is ref ? : {}", root_ref.upgrade().is_some());

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
        println!(
            "{} played: {} with score {}. Tree size is : {} (mate depth : {:?})",
            played_str,
            string_from_move(&best_node.upgrade().unwrap().borrow().get_move().unwrap()),
            best_node.upgrade().unwrap().borrow().get_best_score(),
            tree.size(),
            best_node.upgrade().unwrap().borrow().get_mate_depth()
        );
        for scored_node in nodes.iter().skip(1).take(3) {
            println!(
                "     - also possible: {} with score: {} (mate depth : {:?})",
                string_from_move(&scored_node.upgrade().unwrap().borrow().get_move().unwrap()),
                scored_node.upgrade().unwrap().borrow().get_best_score(),
                scored_node.upgrade().unwrap().borrow().get_mate_depth()
            );
        }

        let _ = tree.select_branch(best_node.upgrade().unwrap().borrow().get_move().unwrap());
        print_board(tree.root().borrow().get_engine().get_board());
        println!("Number of moves available : {}", nodes.len());

        // user input
        let mut incorrect_move = true;
        while incorrect_move {
            let pm = input!(String, "Input a move: ");
            if pm == "moves".to_string() {
                let moves = tree.get_sorted_nodes();
                println!("Incorrect move, please retry : {}", moves.len());
                continue;
            }
            let player_move: PlayerMove;
            match tree.root().borrow().get_engine().get_move_by_str(&pm) {
                Ok(mv) => {
                    player_move = mv;
                }
                Err(()) => {
                    let moves = tree.get_sorted_nodes();
                    println!("Incorrect move, please retry : {}", moves.len());
                    continue;
                }
            }
            match tree.select_branch(player_move) {
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

fn test_mate() {
    // Play move for mat du berger
    let mut engine = Engine::new();
    engine.play(create_move_from_str("e2e4")).unwrap();
    engine.play(create_move_from_str("e7e5")).unwrap();
    engine.play(create_move_from_str("f1c4")).unwrap();
    engine.play(create_move_from_str("a7a6")).unwrap();
    engine.play(create_move_from_str("d1f3")).unwrap();
    engine.play(create_move_from_str("b8c6")).unwrap();

    let mut tree = Tree::new(
        engine,
        Box::new(ValueRuleSet::new()),
        6,
        1e6 as usize,
        f32::INFINITY,
    );

    tree.generate_tree();
    let scored_nodes = tree.get_sorted_nodes();
    println!("Number of moves : {}", scored_nodes.len());
    let best_move = &scored_nodes[0]
        .upgrade()
        .unwrap()
        .borrow()
        .get_move()
        .unwrap();
    println!("Best move : {}", string_from_move(best_move));
    let _ = tree.select_branch(best_move.clone());
    print_board(tree.root().borrow().get_engine().get_board());
}
fn main() {
    // test_mate();
    // drop_branch_test();
    play_against_robot(false, 10, 1e6 as usize);
    // play_robot_to_robot(6, 1e9 as usize, true);
    // let ev = ValueRuleSet {};
    // let e = Engine::new();
    // let r = ev.evaluate(&e.board());
    // println!("{}", r);
    // println!("Base score : {}", tree.root().borrow().recursive_score());
    // for (pm, score ) in tree.get_sorted_moves() {
    //     println!("{} with score : {}", string_from_move(&pm), score);
    // }
}
