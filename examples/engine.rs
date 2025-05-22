use chess_engine::prelude::evaluators::{AdvancedEvaluator, BasicEvaluator};
use chess_engine::prelude::print_board;
use chess_engine::prelude::Engine;
use chess_engine::prelude::TreeSearch;

fn main() {
    // Init the tree
    let evaluator = AdvancedEvaluator::default();
    let mut tree = TreeSearch::new(1e7 as usize, Box::new(evaluator));

    // init the engine
    let mut engine = Engine::new();
    engine
        .play_pgn_str(
            r"1. d4 d5 2. Bf4 Nc6 3. Nf3 Bf5 4. e3 e6 5. Nbd2 Bb4 6. c3 Be7 7. Bb5 a6 8. Bxc6+
bxc6 9. Kf1 c5 10. Qb3 c4 11. Qa4+ c6 12. Qxc6+ Kf8 13. Ne5 Nf6 14. f3 Rc8 15.
Qb7 Rc7 16. Qxa6 Bd3+ 17. Kf2 Ne4+ 18. fxe4 dxe4 19. b3 cxb3 20. axb3 Bxa6 21.
Rxa6 Rxc3 22. Rha1 f6 23. Ra8 Rc8 24. Nc6 Qd7 25. R1a6 Kf7 26. R8a7 Rc6 27. Rd7 Ra6 28. g4 e5
29. e5 e5 30.
    ",
        )
        .unwrap();

    // search best move
    match tree.iterative_search(engine.clone(), 6) {
        Some(mv) => {
            let _ = engine.play(mv);
            print_board(engine.get_board())
        }
        None => println!("Did not find any move"),
    }
}
