pub enum RunningState {
    Normal,
    WhiteChecked,
    BlackChecked,
}

pub enum GameState {
    Running(),
    Mate,
    Stale,
}
