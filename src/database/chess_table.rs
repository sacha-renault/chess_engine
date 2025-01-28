use rusqlite::{named_params, params, Connection, Result};
use std::sync::Mutex;

use super::init::init_db;
use super::move_model::MoveModel;

pub struct ChessTablesDb {
    conn: Mutex<Connection>,
}

impl ChessTablesDb {
    
}