pub mod boards;
pub mod game_engine;
pub mod pieces;
pub mod tree_search;
pub mod database;
pub mod lichess_api;
pub mod static_evaluation;

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

use boards::{Board, ColorBoard};
use lichess_api::api_error;
use core::f32;
use game_engine::debug::print_board;
use game_engine::player_move::PromotionMove;
use game_engine::utility::move_piece;
use pieces::Piece;
use prelude::{
    create_move_from_str, iter_into_u64, string_from_move, Engine, NormalMove, PlayerMove,
};
use static_evaluation::evaluator_trait::Evaluator;
use tree_search::node_with_score::NodeWithScore;
use tree_search::tree::Tree;
use tree_search::tree_builder::TreeBuilder;
use tree_search::tree_node::TreeNode;
use static_evaluation::BasicEvaluator;
use std::cell::RefCell;
use std::{panic, usize};
use std::mem;
use lichess_api::lichess_requests::fetch_lichess_moves;

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

fn play_robot_to_robot(engine: Engine, display: bool) {
    // Create the tree from the engine
    let mut tree = TreeBuilder::new()
        .max_depth(10)
        .max_size(1e6 as usize)
        .foreseeing_windowing(f32::INFINITY)
        .max_quiescence_depth(5)
        .razoring_depth(usize::MAX)
        .razoring_margin_base(-25.)
        .build_tree(engine, Box::new(BasicEvaluator::new()))
        .unwrap();

    let mut i = 0;

    while tree
        .root()
        .borrow()
        .get_engine()
        .get_all_moves_by_piece()
        .len()
        != 0
    {
        let played_str = {
            if tree.root().borrow().get_engine().white_to_play() {
                "White"
            } else {
                "Black"
            }
        };

        let mv_num = tree.root().borrow().get_engine().get_fullmove_number();

        let output = tree.iterative_deepening();
        match output.get_move() {
            Some(mv) => {
                println!(
                    "{} - {} played: {} with score {} (Depth : {}, mate depth : {:?}, tree size : {})",
                    mv_num,
                    played_str,
                    string_from_move(&mv),
                    output.get_score(),
                    output.get_depth(),
                    output.mate_depth(),
                    tree.size()
                );
                let _ = tree.select_branch(mv);
            }
            None => {
                panic!("{}  with score {} (Depth : {}, mate depth : {:?}, tree size : {})",
                    played_str,
                    output.get_score(),
                    output.get_depth(),
                    output.mate_depth(),
                    tree.size());
            },

        }

        i += 1;
        if display {
            print_board(tree.root().borrow().get_engine().get_board());
        }

        // if i > 3 {
        //     break;
        // }
    }
}

fn play_against_robot(engine: Engine) {
    // Create the tree from the engine
    let mut tree = TreeBuilder::new()
        .max_depth(5)
        .max_size(1e6 as usize)
        .foreseeing_windowing(f32::INFINITY)
        .max_quiescence_depth(0)
        .razoring_depth(usize::MAX)
        .razoring_margin_base(-25.)
        .build_tree(engine, Box::new(BasicEvaluator::new()))
        .unwrap();

    loop {
        let played_str = {
            if tree.root().borrow().get_engine().white_to_play() {
                "White"
            } else {
                "Black"
            }
        };

        let output = tree.iterative_deepening();
        println!(
            "{} played: {} with score {} (Depth : {}, mate depth : {:?})",
            played_str,
            string_from_move(&output.get_move().unwrap()),
            output.get_score(),
            output.get_depth(),
            output.mate_depth(),
        );
        let _ = tree.select_branch(output.get_move().unwrap());
        print_board(tree.root().borrow().get_engine().get_board());

        // user input
        let mut incorrect_move = true;
        while incorrect_move {
            let pm = input!(String, "Input a move: ");
            if pm == "moves".to_string() {
                println!("Incorrect move, please retry",);
                continue;
            }
            let player_move: PlayerMove;
            match tree.root().borrow().get_engine().get_move_by_san(&pm) {
                Ok(mv) => {
                    player_move = mv;
                }
                Err(_) => {
                    println!("Incorrect move, please retry");
                    continue;
                }
            }
            match tree.select_branch(player_move) {
                Ok(()) => {
                    incorrect_move = false;
                }
                Err(()) => {
                    println!("Couldn't select branch, please retry");
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

    let mut tree = TreeBuilder::new()
        .max_depth(10)
        .max_size(1e6 as usize)
        .foreseeing_windowing(f32::INFINITY)
        .max_quiescence_depth(0)
        .build_tree(engine, Box::new(BasicEvaluator::new()))
        .unwrap();

    let output = tree.iterative_deepening();

    println!(
        "Played: {} with score {} (Depth : {}, mate depth : {:?})",
        string_from_move(&output.get_move().unwrap()),
        output.get_score(),
        output.get_depth(),
        output.mate_depth(),
    );
    print_board(tree.root().borrow().get_engine().get_board());
}

fn test_debug(engine: Engine) {
    let mut tree = TreeBuilder::new()
        .max_depth(6)
        .max_quiescence_depth(0)
        .max_size(1e6 as usize)
        .foreseeing_windowing(f32::INFINITY)
        .razoring_depth(usize::MAX)
        .build_tree(engine, Box::new(BasicEvaluator::new()))
        .unwrap();

        // tree.select_branch(create_move_from_str("d5c3")).unwrap();
    // tree.select_branch(create_move_from_str("b6f6")).unwrap();
    // tree.select_branch(create_move_from_str("g4g3")).unwrap();
    // tree.select_branch(create_move_from_str("f2f3")).unwrap();
    // tree.select_branch(create_move_from_str("g3g2")).unwrap();

    let output = tree.iterative_deepening();
    println!("Reached depth : {} with tree size : {}", output.get_depth(), tree.size());
    println!("Best move : {} with score {}", string_from_move(&output.get_move().unwrap()), output.get_score());
}

fn main() {
    let mut engine = Engine::new();
    // let pgn = "e4 c5 2. Bc4 d5 3. exd5 Qb6";
    let pgn = "1. b4 e6 2. Bb2 Bxb4 3. Bxg7 Nf6";

    engine.play_pgn_str(pgn).unwrap();
    play_robot_to_robot(engine, false);
    // play_against_robot(engine);
    // print_board(engine.get_board());
    // println!("{}", engine.to_string());

    // println!("White to play : {}", engine.white_to_play());
    // test_debug(engine);
    // test_mate();
    //
    // match fetch_lichess_moves(&engine.to_string(), "") {
    //     Ok(moves) => {
    //         for m in moves {
    //             println!("Moves : {:?}", m)
    //         }
    //     },
    //     Err(err) => println!("Error : {:?}", err)
    // }

    // test_mate();
    // drop_branch_test();

    // play_robot_to_robot(6, 1e9 as usize, true);
    // let ev = ValueRuleSet {};
    // let e = Engine::new();
    // let r = ev.evaluate(&e.board());
    // println!("{}", r);
    // println!("Base score : {}", tree.root().borrow().recursive_score());
    // for (pm, score ) in tree.get_sorted_moves() {
    //     println!("{} with score : {}", string_from_move(&pm), score);
    // }
    // println!("Board size: {}", mem::size_of::<Board>());
    // println!("ColorBoard size: {}", mem::size_of::<ColorBoard>());
    // println!("TreeNode size: {}", mem::size_of::<TreeNode>());
}
