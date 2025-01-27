use std::path::PathBuf;

use directories_next::ProjectDirs;

pub fn get_db_path() -> PathBuf {
    // App's parameters
    let qualifier = "com";
    let organization = "sacha-renault";
    let application = "chess_engine";

    // Get the platform-specific data directory
    if let Some(proj_dirs) = ProjectDirs::from(qualifier, organization, application) {
        let data_dir = proj_dirs.data_dir(); // Path to the data directory
        println!("Data directory: {:?}", data_dir);

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(data_dir).expect("Failed to create data directory");

        // Example: Store a SQLite database in the data directory
        let db_path = data_dir.join("chess_tables.sqlite");
        db_path
    } else {
        panic!("Could not determine the data directory.");
    }
}

pub fn init_db() {
    let db_path = get_db_path();
    println!("Database path: {:?}", db_path);

    // Open the database
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    // Create tables
    conn.execute(
        "CREATE TABLE boards (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            fen TEXT UNIQUE NOT NULL
        );

        CREATE TABLE moves (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            board_id INTEGER NOT NULL,
            move TEXT NOT NULL,
            win_rate REAL NOT NULL,
            FOREIGN KEY (board_id) REFERENCES boards (id) ON DELETE CASCADE
        );",
        [],
    )
    .expect("Failed to create games table");
}