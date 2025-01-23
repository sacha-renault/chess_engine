use crate::{boards::Board, pieces::Piece};

pub struct PieceMoveOutput {
    pub board: Board,
    pub captured_piece: Option<Piece>,
}