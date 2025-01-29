use crate::game_engine::engine::Engine;
use crate::game_engine::player_move::PlayerMove;
use crate::pieces::Piece;

pub trait Evaluator {
    fn evaluate_engine_state(&self, engine: &Engine, depth: usize) -> f32;

    fn evaluate_heuristic_move(
        &self,
        player_move: PlayerMove,
        moved_piece: Piece,
        captured_piece_opt: Option<Piece>,
        is_king_checked: bool
    ) -> f32;
}
