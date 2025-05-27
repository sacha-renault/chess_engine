use crate::prelude::PlayerMove;
use std::collections::HashMap;

/// Type of bound stored in the transposition table entry
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundType {
    /// Exact score (PV node)
    Exact,
    /// Lower bound (beta cutoff, fail-high)
    LowerBound,
    /// Upper bound (fail-low, all moves searched without improving alpha)
    UpperBound,
}

/// Entry in the transposition table
#[derive(Debug, Clone)]
pub struct TTEntry {
    /// Zobrist hash of the position
    pub hash: u64,
    /// Best move found for this position
    pub best_move: Option<PlayerMove>,
    /// Evaluation score
    pub score: f32,
    /// Search depth when this entry was created
    pub depth: usize,
    /// Type of bound (exact, lower, upper)
    pub bound_type: BoundType,
    /// Age/generation for replacement scheme
    pub age: u32,
}

impl TTEntry {
    pub fn new(
        hash: u64,
        best_move: Option<PlayerMove>,
        score: f32,
        depth: usize,
        bound_type: BoundType,
        age: u32,
    ) -> Self {
        Self {
            hash,
            best_move,
            score,
            depth,
            bound_type,
            age,
        }
    }
}

/// Transposition Table for storing search results
pub struct TranspositionTable {
    /// The actual table storing entries
    table: HashMap<u64, TTEntry>,
    /// Maximum number of entries (for memory management)
    max_size: usize,
    /// Current age/generation for replacement scheme
    current_age: u32,
    /// Statistics
    hits: u64,
    misses: u64,
}

impl TranspositionTable {
    /// Create a new transposition table with given capacity
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            table: HashMap::with_capacity(max_size),
            max_size,
            current_age: 0,
            hits: 0,
            misses: 0,
        }
    }

    /// Create with default capacity (1M entries ≈ 64MB)
    pub fn with_default_capacity() -> Self {
        Self::with_capacity(1_000_000)
    }

    /// Probe the transposition table for a position
    pub fn probe(
        &mut self,
        hash: u64,
        depth: usize,
        alpha: f32,
        beta: f32,
    ) -> Option<(f32, Option<PlayerMove>)> {
        if let Some(entry) = self.table.get(&hash) {
            // Verify hash collision protection
            if entry.hash != hash {
                self.misses += 1;
                return None;
            }

            // Only use entries from equal or greater depth
            if entry.depth >= depth {
                let score = self.adjust_mate_score_from_tt(entry.score, depth);

                // Check if we can use this score based on bound type
                let can_use = match entry.bound_type {
                    BoundType::Exact => true,
                    BoundType::LowerBound => score >= beta,
                    BoundType::UpperBound => score <= alpha,
                };

                if can_use {
                    self.hits += 1;
                    return Some((score, entry.best_move.clone()));
                }
            }

            // Even if we can't use the score, we can still use the best move for ordering
            if entry.best_move.is_some() {
                self.hits += 1;
                return Some((f32::NAN, entry.best_move.clone())); // NAN signals "use move only"
            }
        }

        self.misses += 1;
        None
    }

    /// Store an entry in the transposition table
    pub fn store(
        &mut self,
        hash: u64,
        best_move: Option<PlayerMove>,
        score: f32,
        depth: usize,
        bound_type: BoundType,
    ) {
        // Adjust mate scores for storage
        let adjusted_score = self.adjust_mate_score_for_tt(score, depth);

        let entry = TTEntry::new(
            hash,
            best_move,
            adjusted_score,
            depth,
            bound_type,
            self.current_age,
        );

        // Check if we need to make room
        if self.table.len() >= self.max_size && !self.table.contains_key(&hash) {
            self.make_room();
        }

        // Store the entry (will replace if key already exists)
        self.table.insert(hash, entry);
    }

    /// Adjust mate scores when storing to TT (relative to current position)
    fn adjust_mate_score_for_tt(&self, score: f32, ply: usize) -> f32 {
        if score.abs() > crate::static_evaluation::values::MATE_THRESHOLD {
            if score > 0.0 {
                score + ply as f32
            } else {
                score - ply as f32
            }
        } else {
            score
        }
    }

    /// Adjust mate scores when retrieving from TT (relative to current position)
    fn adjust_mate_score_from_tt(&self, score: f32, ply: usize) -> f32 {
        if score.abs() > crate::static_evaluation::values::MATE_THRESHOLD {
            if score > 0.0 {
                score - ply as f32
            } else {
                score + ply as f32
            }
        } else {
            score
        }
    }

    /// Make room in the table by removing old/shallow entries
    fn make_room(&mut self) {
        // Simple replacement scheme: remove entries from older generations
        // or with lower depth if same generation
        if let Some((&worst_hash, _)) = self
            .table
            .iter()
            .min_by_key(|(_, entry)| (entry.age, entry.depth))
        {
            self.table.remove(&worst_hash);
        }
    }

    /// Clear the transposition table
    pub fn clear(&mut self) {
        self.table.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Advance to next generation (call at start of new search)
    pub fn new_search(&mut self) {
        self.current_age += 1;
    }

    /// Get hit rate for debugging
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }

    /// Get table statistics
    pub fn stats(&self) -> (usize, u64, u64, f64) {
        (self.table.len(), self.hits, self.misses, self.hit_rate())
    }
}

/// Helper function to determine bound type from alpha-beta search result
pub fn get_bound_type(score: f32, alpha: f32, beta: f32) -> BoundType {
    if score <= alpha {
        BoundType::UpperBound // Fail-low
    } else if score >= beta {
        BoundType::LowerBound // Fail-high (beta cutoff)
    } else {
        BoundType::Exact // PV node
    }
}
