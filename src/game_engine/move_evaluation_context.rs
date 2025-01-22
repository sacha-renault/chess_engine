use super::engine::Engine;
use super::player_move::PlayerMove;
use crate::{
    pieces::{Color, Piece},
    prelude::CorrectMoveResults,
};

#[derive(Debug, Clone)]
pub struct MoveEvaluationContext {
    pub engine: Engine,
    pub player_move: PlayerMove,
    pub color: Color,
    pub result: CorrectMoveResults,
    pub piece: Piece,
    pub captured_piece: Option<Piece>,
}
