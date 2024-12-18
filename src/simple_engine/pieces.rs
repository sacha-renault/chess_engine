#[derive(Debug, Copy, Clone)]
pub enum Pieces {
    King = 6,
    Queen = 5,
    Rook = 4,
    Bishop = 3,
    Knight = 2,
    Pawn = 1,
}

pub const ALL_PIECES: [Pieces; 6] = [
    Pieces::Pawn,
    Pieces::Knight,
    Pieces::Bishop,
    Pieces::Rook,
    Pieces::Queen,
    Pieces::King,
];
