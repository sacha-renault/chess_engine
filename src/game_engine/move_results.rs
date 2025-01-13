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
    IllegalMove,
    CastlingNotAllowed,
    WaitingForPromotion,
    IllegalPromotion,
}

pub type MoveResult = Result<CorrectMoveResults, IncorrectMoveResults>;
