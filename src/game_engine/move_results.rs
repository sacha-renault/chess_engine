#[derive(Debug, Copy, Clone)]
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
    PromotionExpected,
    IllegalPromotion,
    InvalidMove
}

pub type MoveResult = Result<CorrectMoveResults, IncorrectMoveResults>;
