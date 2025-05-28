use chess_engine::game_engine::move_results::MoveResult;
use chess_engine::prelude::evaluators::{AdvancedEvaluator, BasicEvaluator};
use chess_engine::prelude::Engine;
use chess_engine::prelude::TreeSearch;
use chess_engine::prelude::{print_board, string_from_move};
use chess_engine::tree_search_v2::tree::TreeSearchBuilder;

use std::collections::HashMap;
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

#[allow(dead_code)]
fn play_against_robot(mut engine: Engine, mut tree: TreeSearch) {
    loop {
        let played_str = {
            if engine.white_to_play() {
                "White"
            } else {
                "Black"
            }
        };

        let next_move = tree.iterative_search(engine.clone()).unwrap();
        let _ = engine.play(*next_move.best_move());

        print_board(engine.get_board());

        println!("{} {}", played_str, next_move);

        // user input
        let mut incorrect_move = true;
        while incorrect_move {
            let pm = input!(String, "Input a move: ");
            if pm == "moves".to_string() {
                println!("Incorrect move, please retry",);
                continue;
            }
            match engine.play_san(&pm) {
                MoveResult::Ok(_) => {
                    incorrect_move = false;
                }
                MoveResult::Err(err) => {
                    println!("Incorrect move: {:?}, please retry", err);
                }
            }
        }
    }
}

// fn play_robot_vs_robot_same_engine(
//     mut engine: Engine,
//     mut tree: TreeSearch,
//     white_depth: u8,
//     black_depth: u8,
// ) {
//     loop {
//         let played_str = {
//             if engine.white_to_play() {
//                 "White"
//             } else {
//                 "Black"
//             }
//         };

//         // Use different search depths for variety
//         let search_depth = if engine.white_to_play() {
//             white_depth
//         } else {
//             black_depth
//         };

//         let next_move = tree.iterative_search(engine.clone(), search_depth).unwrap();
//         let _ = engine.play(next_move);

//         print_board(engine.get_board());

//         println!("{} played: {}", played_str, string_from_move(&next_move));

//         // Optional: Add a small delay to make the game watchable
//         // std::thread::sleep(std::time::Duration::from_millis(1000));

//         // Check for game end conditions
//         if engine.get_all_moves_by_piece().is_empty() {
//             println!("Game over!");
//             break;
//         }
//     }
// }

fn main() {
    // Init the tree
    let evaluator = AdvancedEvaluator::default();
    let tree = TreeSearchBuilder::default()
        .pool_capacity(1e7 as usize)
        .tt_capacity(1e7 as usize)
        .evaluator(Box::new(evaluator))
        .max_depth(10)
        .max_q_depth(0)
        .window_margin(0.)
        .build()
        .unwrap();

    // init the engine
    let mut engine = Engine::new();
    engine
        .play_pgn_str(
            r"1. d4 Nf6 2. Bf4 g6 3. a4 e5 4. Bxe5 h6 5. b4 d6 6. Bxf6 Qxf6 7. e4 c5 8. c3 Nc6
    9. g4 Bd7 10. h4 cxd4 11. cxd4 Qxd4 12. Qxd4 Nxd4 13. e5 dxe5 14. Bg2 Bxb4+ 15.
    Nd2 Bc3 Rc1 e4 f3 Nc2", // f3 Nc2 Rc2 Bb2
        )
        .unwrap();

    // search best move
    play_against_robot(engine, tree);
    // play_robot_vs_robot_same_engine(engine, tree, 8, 8);
}
