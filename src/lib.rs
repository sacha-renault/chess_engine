pub mod boards;
pub mod game_engine;
pub mod pieces;

// Define the prelude module
pub mod prelude {
    pub use super::game_engine::engine::Engine;
    pub use super::game_engine::move_results::{CorrectMoveResults, IncorrectMoveResults};
    pub use super::game_engine::player_move::{CastlingMove, NormalMove, PlayerMove};
    pub use super::game_engine::utility::{coordinates_to_u64, u64_to_coordinates};
}
