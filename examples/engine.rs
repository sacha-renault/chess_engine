use chess_engine::game_engine::move_results::MoveResult;
use chess_engine::prelude::evaluators::{AdvancedEvaluator, BasicEvaluator};
use chess_engine::prelude::Engine;
use chess_engine::prelude::TreeSearch;
use chess_engine::prelude::{print_board, string_from_move};

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

        let next_move = tree.iterative_search(engine.clone(), 8).unwrap();
        let _ = engine.play(next_move);

        print_board(engine.get_board());

        println!("{} played: {}", played_str, string_from_move(&next_move));

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

fn main() {
    // Init the tree
    let evaluator = BasicEvaluator::new();
    let tree = TreeSearch::new(1e7 as usize, Box::new(evaluator));

    // init the engine
    let mut engine = Engine::new();
    engine
        .play_pgn_str(
            r"1. Nc3 d5 2. e3 e5 3. Qh5 Nf6 4. Bb5+ c6 5. Qxe5+ Be7 6. Bd3 Nbd7 7. Qg5 h6 8.
Qxg7 Rg8 9. Qxh6 Ne5 10. Qf4 Bd6 11. Qh4 Nxd3+ 12. cxd3 Qe7 13. Nge2 Nd7 14. Qh7
Nf6 15. Qh4 Rg4 16. Qh3 Bf5 17. e4 Bc8 18. exd5 Rg6 19. Qe3 Nxd5 20. Nxd5 cxd5
21. Qd4 Rxg2 22. h3 Bc5 23. Qh8+ Qf8 24. Qxf8+ Kxf8 25. b3 Bf5 26. a3 Re8 27.
Kd1 Bxd3 Nf4 Bd4 Ng2 Be2 Kc2 Rc8 Kb1 Bd3 Ka2 Rc2 Kb1 Rc3 Kb2 Rc2 Kb1",
        )
        .unwrap();

    // search best move
    play_against_robot(engine, tree);
}
