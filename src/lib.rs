pub mod boards;
pub mod database;
pub mod game_engine;
pub mod lichess_api;
pub mod pieces;
pub mod static_evaluation;
pub mod tree_search;
pub mod smart_engine;

pub mod prelude {
    // Usefull struct
    pub use crate::boards::Board;
    pub use crate::game_engine::engine::Engine;
    pub use crate::game_engine::player_move::PlayerMove;
    pub use crate::pieces::{Piece, Color};
    pub use crate::static_evaluation::evaluators;
    pub use crate::tree_search::tree::Tree;
    pub use crate::tree_search::tree_builder::TreeBuilder;
    pub use crate::database::chess_table::ChessTablesDb;

    // Usefull functions
    pub use crate::game_engine::utility::string_from_move;
    pub use crate::game_engine::debug::{print_board, print_bitboard};

    // usefull module
    pub use crate::lichess_api::lichess_requests;
}