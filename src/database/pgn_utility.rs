use regex::Regex;
use sha2::{Sha256, Digest};

use crate::game_engine::engine::Engine;

pub fn hash_pgn(pgn: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(pgn);
    hasher.finalize().iter().copied().collect()
}

/// TODO
/// This function takes a PGN game, removes the initial infos in between []
/// Then it get who wins (black or white)
/// Once this is done, it iterates over each moves
/// And feed a vec
pub fn parse_pgn(pgn: &str, max_moves: usize) -> Result<Vec<(String, String, f32)>, ()> {
    // Remove metadata inside []
    let re = Regex::new(r"\[.*?\]").unwrap();
    let pgn = re.replace_all(pgn, "");

    // Get the result of the game (Who won?)
    let (white_res, black_res) = if pgn.contains("1-0") {
        (1., 0.)
    } else if pgn.contains("0-1") {
        (0., 1.) // Black wins
    } else if pgn.contains("1/2-1/2") {
        (0.5, 0.5)  // Draw
    } else {
        return Err(()); // Game isn't finished ?
    };

    // Split the moves based on spaces, removing result strings
    let moves: Vec<&str> = pgn.split_whitespace()
        .filter(|token| !token.ends_with('.') && !token.is_empty())
        .filter(|&move_str| !move_str.contains("1-0") && !move_str.contains("0-1") && !move_str.contains("1/2-1/2"))
        .collect();

    // Init move vec
    let mut move_vec = Vec::new();

    // init a boolean that will swap at every iteration
    let mut white_to_play = true;

    // init an engine to test the validity of the moves
    let mut engine = Engine::new();

    for (i, str_mv) in moves.iter().enumerate() {
        if i / 2 > max_moves {
            break;
        }

        let fen = engine.to_string();
        let mv = engine.get_move_by_str(str_mv)?;
        engine.play(mv).map_err(|_| ())?;

        if white_to_play {
            move_vec.push((fen, str_mv.to_string(), white_res));
        } else {
            move_vec.push((fen, str_mv.to_string(), black_res));
        }

        white_to_play = !white_to_play;
    }

    Ok(move_vec)
}