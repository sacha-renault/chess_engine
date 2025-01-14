use super::engine::Engine;
use super::player_move::PlayerMove;
use crate::pieces::{Color, Piece};

#[derive(Debug)]
pub struct GetMoveRow {
    pub engine: Engine,
    pub player_move: PlayerMove,
    pub piece: Piece,
    pub color: Color,
}