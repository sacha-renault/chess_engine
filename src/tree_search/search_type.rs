/// Represents the type of search performed in a chess engine.
///
/// Each variant carries a `usize` value that specifies the search depth.
pub enum SearchType {
    /// A full search explores all possible moves up to the specified depth.
    Full,

    /// A quiescence search focuses on resolving "noisy" positions (e.g., captures, checks)
    /// until the position becomes stable.
    Quiescence(usize),
}

