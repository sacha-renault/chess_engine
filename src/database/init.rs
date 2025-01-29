use std::path::PathBuf;

use directories_next::ProjectDirs;
use rusqlite::Connection;

/// Get the default path for the database
pub fn get_db_path() -> PathBuf {
    // App's parameters
    let qualifier = "com";
    let organization = "sacha-renault";
    let application = "chess_engine";

    // Get the platform-specific data directory
    if let Some(proj_dirs) = ProjectDirs::from(qualifier, organization, application) {
        let data_dir = proj_dirs.data_dir(); // Path to the data directory

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(data_dir).expect("Failed to create data directory");

        // Example: Store a SQLite database in the data directory
        let db_path = data_dir.join("chess_tables.sqlite");
        db_path
    } else {
        panic!("Could not determine the data directory.");
    }
}

/// Initialize the database connection
pub fn init_db(db_path: &PathBuf) -> Result<Connection, rusqlite::Error> {
    // Open the database
    let conn = Connection::open(db_path)?;

    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS boards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            fen TEXT UNIQUE NOT NULL
        );",
        []
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS moves (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            board_id INTEGER NOT NULL,
            san TEXT UNIQUE NOT NULL,
            win_rate REAL NOT NULL,
            draw_rate REAL NOT NULL,
            loose_rate REAL NOT NULL,
            game_number INTEGER NOT NULL,
            FOREIGN KEY (board_id) REFERENCES boards (id) ON DELETE CASCADE
        );",
        []
    )?;

    Ok(conn)
}