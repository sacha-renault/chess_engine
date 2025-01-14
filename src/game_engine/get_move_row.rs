use super::engine::Engine;
use super::player_move::PlayerMove;
use crate::{pieces::{Color, Piece}, prelude::CorrectMoveResults};

#[derive(Debug, Clone)]
pub struct GetMoveRow {
    pub engine: Engine,
    pub player_move: PlayerMove,
    pub piece: Piece,
    pub color: Color,
    pub result: CorrectMoveResults
}