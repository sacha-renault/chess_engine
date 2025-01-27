use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::IntoIter;

pub struct PgnGame {
    pub event: String,
    pub white_elo: u32,
    pub black_elo: u32,
    pub game: String,
}

pub struct PgnGameIterator {
    lines: Vec<String>, // store all lines to process them one by one
    current_line: usize, // track the current line in the iterator
}

impl PgnGameIterator {
    // Constructor to create the iterator from a file
    pub fn new(pgn_path: &str) -> io::Result<Self> {
        let file = File::open(pgn_path)?;
        let reader = io::BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
        Ok(PgnGameIterator {
            lines,
            current_line: 0,
        })
    }
}

impl Iterator for PgnGameIterator {
    type Item = PgnGame;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pgn_game = String::new();
        let mut event = String::new();
        let mut white_elo = None;
        let mut black_elo = None;
        let mut moves = String::new();

        // Collect the metadata for the game (event, Elo ratings, etc.)
        while self.current_line < self.lines.len() {
            let line = &self.lines[self.current_line];
            self.current_line += 1;

            // Handle game metadata (Event, Elo ratings, etc.)
            if line.starts_with("[Event") {
                event = line
                    .trim_start_matches("[Event \"")
                    .trim_end_matches("\"]")
                    .to_string();
            }
            if line.starts_with("[WhiteElo") {
                if let Some(elo) = line.split('"').nth(1) {
                    white_elo = Some(elo.parse::<u32>().unwrap_or_default());
                }
            }
            if line.starts_with("[BlackElo") {
                if let Some(elo) = line.split('"').nth(1) {
                    black_elo = Some(elo.parse::<u32>().unwrap_or_default());
                }
            }
            if line == "" { // Empty line marks the end of metadata section
                break;
            }
        }

        // Now collect the game moves
        while self.current_line < self.lines.len() {
            let line = &self.lines[self.current_line];
            self.current_line += 1;
            if line.starts_with("[") { // New game metadata starts, we stop here
                self.current_line -= 1; // Unconsume this line for the next iteration
                break;
            }
            moves.push_str(&line); // Accumulate the game moves
        }

        // If required fields are missing, return None
        if event.is_empty() || white_elo.is_none() || black_elo.is_none() || moves.is_empty() {
            return None; // Skip incomplete games
        }

        // Return the captured PGN game
        Some(PgnGame {
            event,
            white_elo: white_elo.unwrap(),
            black_elo: black_elo.unwrap(),
            game: moves.trim().to_string(),
        })
    }
}
