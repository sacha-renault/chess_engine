pub mod simple_engine; // Ensure this line is present

// Define the prelude module
pub mod prelude {
    pub use super::simple_engine::board::Board;
    pub use super::simple_engine::engine::Engine;
    pub use super::simple_engine::move_results::{CorrectMoveResults, IncorrectMoveResults};
    pub use super::simple_engine::player_move::{CastlingMove, NormalMove, PlayerMove};
    pub use super::simple_engine::utility::{coordinates_to_u64, u64_to_coordinates};
}
