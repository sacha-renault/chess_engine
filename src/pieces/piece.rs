#[derive(Debug, Copy, Clone)]
pub enum Piece {
    King = 6,
    Queen = 5,
    Rook = 4,
    Bishop = 3,
    Knight = 2,
    Pawn = 1,
}

pub const ALL_PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

pub const PROMOTE_PIECE: [Piece; 4] = [
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
];
