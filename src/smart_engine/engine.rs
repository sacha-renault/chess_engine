use crate::tree_search::tree::Tree;
use crate::database::chess_table::ChessTablesDb;

use super::config::EngineConfig;

pub struct SmartEngine {
    tree: Tree,
    config: EngineConfig,
    db: ChessTablesDb,
}

impl SmartEngine {
    pub fn new(tree: Tree, config: EngineConfig) -> Result<Self, ()> {
        let db = match &config.db_path {
            Some(db_path) => ChessTablesDb::at_path(db_path.to_path_buf()),
            None => ChessTablesDb::new() 
        }.map_err(|_| ())?;
        Ok(SmartEngine {
            tree,
            config, 
            db
        })
    }
}