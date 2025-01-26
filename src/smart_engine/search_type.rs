/// Represents the type of search performed in a chess engine.
///
/// Each variant carries a `usize` value that specifies the search depth.
pub enum SearchType {
    /// A full search explores all possible moves up to the specified depth.
    /// 
    /// The `usize` value represents the depth to which the search is conducted.
    Full(usize),
    
    /// A foreseeing search performs a limited exploration to estimate future positions.
    /// 
    /// The `usize` value specifies the depth of this search.
    Foreseeing(usize),
    
    /// A quiescence search focuses on resolving "noisy" positions (e.g., captures, checks)
    /// until the position becomes stable.
    ///
    /// The `usize` value indicates the depth limit for this search.
    Quiescence(usize),
}

