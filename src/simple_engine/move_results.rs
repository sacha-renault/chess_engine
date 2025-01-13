#[derive(Debug)]
pub enum CorrectMoveResults {
    Ok,
    Promote,
    Check,
    Mate,
    Draw,
}

#[derive(Debug)]
pub enum IncorrectMoveResults {
    KingStillChecked,
    NoPieceAtLocation,
    IncorrectMove,
    CastlingNotAllowed,
    WaitingForPromotion,
}

pub type MoveResult = Result<CorrectMoveResults, IncorrectMoveResults>;
