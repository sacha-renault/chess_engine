pub mod boards;
pub mod database;
pub mod game_engine;
pub mod lichess_api;
pub mod pieces;
pub mod static_evaluation;
pub mod tree_search_v2;

pub mod prelude {
    // Usefull struct
    pub use crate::boards::Board;
    pub use crate::database::chess_table::ChessTablesDb;
    pub use crate::game_engine::engine::Engine;
    pub use crate::game_engine::player_move::PlayerMove;
    pub use crate::pieces::{Color, Piece};
    pub use crate::static_evaluation::evaluators;
    pub use crate::tree_search_v2::tree::TreeSearch;
    // pub use crate::smart_engine::config::EngineConfig;
    // pub use crate::smart_engine::engine::SmartEngine;
    // pub use crate::smart_engine::next_move::MoveEvaluation;
    // pub use crate::tree_search::tree::Tree;
    // pub use crate::tree_search::tree_builder::TreeBuilder;

    // Usefull functions
    pub use crate::game_engine::debug::{print_bitboard, print_board};
    pub use crate::game_engine::utility::string_from_move;

    // usefull module
    pub use crate::lichess_api::lichess_requests;
}
