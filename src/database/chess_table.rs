use rusqlite::{named_params, params, Connection, Result};
use std::sync::Mutex;

use super::init::init_db;
use super::move_model::MoveModel;
use super::pgn_game::PgnGameIterator;
use super::pgn_utility::{hash_pgn, parse_pgn};

pub struct ChessTablesDb {
    conn: Mutex<Connection>,
}

impl ChessTablesDb {
    pub fn new() -> Result<Self> {
        let conn = init_db()?;
        Ok(ChessTablesDb {
            conn: Mutex::new(conn),
        })
    }

    pub fn populate_database_by_book(&self, pgn_book_path: &str) {
        // Open the pgn book as an iterator
        match PgnGameIterator::new(pgn_book_path) {
            Ok(it) => {
                for (index, game) in it.enumerate() {
                    let _ = self.populate_database_by_pgn(&game.game, None);
                    println!("On more game in db : {}", index);
                }
            }
            Err(_) => { }
        }
    }

    pub fn populate_database_by_pgn(&self, pgn: &str, max_moves_opt: Option<usize>) -> Result<(), ()> {
        // Get pgn hash and check if it exist in the db
        let pgn_hash = hash_pgn(pgn);
        if !self.insert_pgn_hash(pgn_hash).map_err(|_| ())? {
            return Ok(()); // already exists in db
        }

        // We parse the pgn
        let result = parse_pgn(pgn, max_moves_opt.unwrap_or(usize::MAX))?;

        // For each result, insert the FEN and move
        for (fen, move_str, res) in result {
            // Insert FEN and get the board_id
            let board_id = self.insert_fen(&fen).map_err(|_| ())?;

            // Insert the move
            self.insert_move(&move_str, board_id, res).map_err(|_| ())?;
        }

        Ok(())
    }

    fn insert_fen(&self, fen: &str) -> Result<i64> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let mut stmt = conn.prepare(
            "INSERT OR IGNORE INTO boards (fen) VALUES (?1)",
        )?;

        // Execute the insert, returning the last inserted row id
        stmt.execute(params![fen])?;

        // Get the board_id (row id of the inserted or existing record)
        let board_id: i64 = conn.query_row(
            "SELECT id FROM boards WHERE fen = ?1",
            params![fen],
            |row| row.get(0),
        )?;

        Ok(board_id)
    }

    fn insert_move(&self, move_text: &str, board_id: i64, result: f32) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        // Check if the move already exists
        let mut stmt = conn.prepare(
            "SELECT id, win_rate, game_number FROM moves WHERE board_id = ?1 AND move = ?2",
        )?;

        let mut rows = stmt.query(params![board_id, move_text])?;

        if let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let current_win_rate: f32 = row.get(1)?;
            let game_number: i32 = row.get(2)?;

            // Update the existing move
            let updated_win_rate = (current_win_rate * game_number as f32 + result) / (game_number + 1) as f32;
            let updated_game_number = game_number + 1;

            conn.execute(
                "UPDATE moves SET win_rate = ?1, game_number = ?2 WHERE id = ?3",
                params![updated_win_rate, updated_game_number, id],
            )?;
        } else {
            // Insert a new move if it doesn't exist
            conn.execute(
                "INSERT INTO moves (board_id, move, win_rate, game_number) VALUES (?1, ?2, ?3, ?4)",
                params![board_id, move_text, result, 1],  // game_number starts at 1
            )?;
        }

        Ok(())
    }


    fn insert_pgn_hash(&self, pgn_hash: Vec<u8>) -> Result<bool> {
        // Insert the pgn_hash into the games table, ignore if it already exists
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
        let affected_rows = conn.execute(
            "INSERT OR IGNORE INTO games (pgn_hash) VALUES (?1)",
            params![pgn_hash],
        )?;

        // If the affected rows is 0, it means the record already existed (no new rows were inserted)
        Ok(affected_rows > 0)
    }

    pub fn get_moves_by_fen(&self, fen: String) -> Result<Vec<MoveModel>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        let moves = conn
            .prepare(
                "SELECT moves.id, moves.board_id, moves.move, moves.win_rate
                FROM moves
                JOIN boards ON moves.board_id = boards.id
                WHERE boards.fen = :fen
                ORDER BY win_rate DESC",
            )?
            .query_map(named_params! { ":fen": fen }, |row| {
                Ok(MoveModel {
                    id: row.get(0)?,
                    board_id: row.get(1)?,
                    move_text: row.get(2)?,
                    win_rate: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(moves)
    }

    pub fn get_best_move_by_fen(&self, fen: String) -> Result<Option<MoveModel>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        let mut stmt = conn.prepare(
            "SELECT moves.id, moves.board_id, moves.move, moves.win_rate
            FROM moves
            JOIN boards ON moves.board_id = boards.id
            WHERE boards.fen = :fen
            ORDER BY win_rate DESC
            LIMIT 1",
        )?;

        let best_move = stmt.query_row(named_params! { ":fen": fen }, |row| {
            Ok(MoveModel {
                id: row.get(0)?,
                board_id: row.get(1)?,
                move_text: row.get(2)?,
                win_rate: row.get(3)?,
            })
        });

        match best_move {
            Ok(mv) => Ok(Some(mv)), // Return the move if found
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None), // No result found
            Err(e) => Err(e), // Propagate other errors
        }
    }

    pub fn get_moves_by_page(&self, page: usize, page_size: usize) -> Result<Vec<MoveModel>> {
        let conn = self.conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;

        // Calculate the offset
        let offset = (page - 1) * page_size;

        // Prepare the SQL query with LIMIT and OFFSET
        let mut stmt = conn.prepare(
            "SELECT moves.id, moves.board_id, moves.move, moves.win_rate
            FROM moves
            ORDER BY moves.id
            LIMIT ?1 OFFSET ?2"
        )?;

        // Execute the query and map the results to MoveModel
        let move_iter = stmt.query_map(params![page_size as i32, offset as i32], |row| {
            Ok(MoveModel {
                id: row.get(0)?,
                board_id: row.get(1)?,
                move_text: row.get(2)?,
                win_rate: row.get(3)?,
            })
        })?;

        // Collect the results into a vector
        let moves: Result<Vec<_>, _> = move_iter.collect();

        moves
    }
}