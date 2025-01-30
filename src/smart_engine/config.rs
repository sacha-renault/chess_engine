use std::path::PathBuf;

// smart_engine/config.rs
pub struct EngineConfig {
    pub min_database_games: u32,
    pub database_reliability_threshold: f32,
    pub lichess_api_key: Option<String>,
    pub db_path: Option<PathBuf>
}