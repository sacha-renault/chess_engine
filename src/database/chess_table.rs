use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

use super::init::{init_db, get_db_path};
use super::models::MoveModel;

use crate::lichess_api::models::LichessMove;

/// Database interface for storing and retrieving chess positions and moves
/// 
/// Provides functionality to store chess positions in FEN notation and their associated moves,
/// along with statistical information about move outcomes from real games.
pub struct ChessTablesDb {
    conn: Mutex<Connection>,
    db_path: PathBuf
}

impl ChessTablesDb {
    /// Creates a new database connection using the default database path
    /// 
    /// # Returns
    /// * `Result<Self>` - New database connection or SQLite error
    pub fn new() -> Result<Self, rusqlite::Error> {
        let db_path = get_db_path();
        let conn = init_db(&db_path)?;
        Ok(Self {
            conn: Mutex::new(conn),
            db_path
        })
    }

    /// Creates a new database connection using a specified path
    /// 
    /// # Arguments
    /// * `db_path` - Custom path to the SQLite database file
    /// 
    /// # Returns
    /// * `Result<Self>` - New database connection or SQLite error
    pub fn at_path(db_path: PathBuf) -> Result<Self, rusqlite::Error> {
        let conn = init_db(&db_path)?;
        Ok(Self {
            conn: Mutex::new(conn),
            db_path
        })
    }

    /// Returns the current database file path
    /// 
    /// # Returns
    /// * `&PathBuf` - Reference to the database path
    pub fn get_db_path(&self) -> &PathBuf {
        &self.db_path
    }

    /// Inserts a chess position into the database or returns existing ID
    /// 
    /// # Arguments
    /// * `fen` - The FEN string representing the chess position
    /// 
    /// # Returns
    /// * `Result<i64>` - Board ID in the database or SQLite error
    pub fn insert_board(&self, fen: &str) -> Result<i64, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        
        // First try to get existing board
        if let Ok(board_id) = conn.query_row(
            "SELECT id FROM boards WHERE fen = ?",
            params![fen],
            |row| row.get(0),
        ) {
            return Ok(board_id);
        }

        // If not found, insert new board
        conn.execute(
            "INSERT INTO boards (fen) VALUES (?)",
            params![fen],
        )?;
        
        Ok(conn.last_insert_rowid())
    }

    /// Stores multiple chess moves with their statistics for a given board position
    /// 
    /// # Arguments
    /// * `chess_moves` - Vector of moves with their statistics
    /// * `board_id` - ID of the board position these moves belong to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or SQLite error
    /// 
    /// # Note
    /// Updates existing moves if they already exist for the given position
    pub fn insert_moves(&self, chess_moves: Vec<LichessMove>, board_id: i64) -> Result<(), rusqlite::Error> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;

        for chess_move in chess_moves {
            let total_games = chess_move.white + chess_move.black + chess_move.draws;
            tx.execute(
                "INSERT INTO moves (board_id, san, win_rate, draw_rate, loose_rate, game_number)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(san) DO UPDATE SET 
                    win_rate = ?3,
                    draw_rate = ?4,
                    loose_rate = ?5,
                    game_number = ?6",
                params![
                    board_id,
                    chess_move.san,
                    chess_move.white as f64 / total_games as f64,
                    chess_move.draws as f64 / total_games as f64,
                    chess_move.black as f64 / total_games as f64,
                    total_games,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Inserts both a board position and its associated moves in one transaction
    /// 
    /// # Arguments
    /// * `fen` - The FEN string representing the chess position
    /// * `chess_moves` - Vector of moves with their statistics
    /// 
    /// # Returns
    /// * `Result<i64>` - Board ID in the database or SQLite error
    pub fn insert_board_with_moves(&self, fen: &str, chess_moves: Vec<LichessMove>) -> Result<i64, rusqlite::Error> {
        let board_id = self.insert_board(&fen)?;
        self.insert_moves(chess_moves, board_id)?;
        Ok(board_id)
    }

    /// Retrieves all stored moves for a given chess position
    /// 
    /// # Arguments
    /// * `fen` - The FEN string representing the chess position
    /// 
    /// # Returns
    /// * `Result<Vec<MoveModel>>` - List of moves with their statistics or SQLite error
    pub fn get_moves_by_fen(&self, fen: &str) -> Result<Vec<MoveModel>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT m.id, m.board_id, m.san, m.win_rate, m.draw_rate, m.loose_rate, m.game_number
            FROM moves m
            JOIN boards b ON m.board_id = b.id
            WHERE b.fen = ?"
        )?;

        let moves = stmt.query_map(params![fen], |row| {
            Ok(MoveModel {
                id: Some(row.get(0)?),
                board_id: row.get(1)?,
                san: row.get(2)?,
                win_rate: row.get(3)?,
                draw_rate: row.get(4)?,
                loose_rate: row.get(5)?,
                game_number: row.get(6)?,
            })
        })?;

        let mut result = Vec::new();
        for move_result in moves {
            result.push(move_result?);
        }

        Ok(result)
    }
}