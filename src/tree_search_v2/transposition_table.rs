use crate::prelude::PlayerMove;
use crate::static_evaluation::values;
use std::collections::HashMap;

/// Type of bound stored in the transposition table entry
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Clone)]
pub enum ProbeResult {
    Score(f32),
    Move(PlayerMove),
    Miss,
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
    /// Search depth when this entry was created (remaining depth from this position)
    pub depth: usize,
    /// Ply from root when this entry was created
    pub ply: usize,
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
        ply: usize,
        bound_type: BoundType,
        age: u32,
    ) -> Self {
        Self {
            hash,
            best_move,
            score,
            depth,
            ply,
            bound_type,
            age,
        }
    }
}

/// Transposition Table for storing search results
pub struct TranspositionTable {
    table: HashMap<u64, TTEntry>,
    max_size: usize,
    current_age: u32,
    hits: u64,
    misses: u64,
}

impl TranspositionTable {
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            table: HashMap::with_capacity(max_size),
            max_size,
            current_age: 0,
            hits: 0,
            misses: 0,
        }
    }

    pub fn with_default_capacity() -> Self {
        Self::with_capacity(1_000_000)
    }

    /// Probe the transposition table for a position
    ///
    /// # Parameters
    /// * `hash` - Zobrist hash of the position
    /// * `depth` - Remaining depth needed for this search
    /// * `ply` - Distance from root (for mate score adjustment)
    /// * `alpha`, `beta` - Alpha-beta bounds
    pub fn probe(
        &mut self,
        hash: u64,
        depth: usize,
        ply: usize,
        alpha: f32,
        beta: f32,
    ) -> ProbeResult {
        if (self.hits + self.misses) % 100000 == 0 {
            println!("Probing hash: {:x} at depth {} ply {}", hash, depth, ply);

            // Check if we're storing this position
            match self.table.get(&hash) {
                Some(entry) => {
                    println!(
                        "  Found entry: depth={}, ply={}, our_depth={}, our_ply={}",
                        entry.depth, entry.ply, depth, ply
                    )
                }
                None => println!("  No entry found"),
            }
        }

        if let Some(entry) = self.table.get(&hash) {
            // Check if stored search was deep enough
            let has_usable_score = entry.depth >= depth;

            let score = if has_usable_score {
                Some(self.adjust_mate_score_from_tt(entry.score, ply))
            } else {
                None
            };

            // Check if we can use this score based on bound type
            let score_is_cutoff = if let Some(s) = score {
                match entry.bound_type {
                    BoundType::Exact => true,
                    BoundType::LowerBound => s >= beta,
                    BoundType::UpperBound => s <= alpha,
                }
            } else {
                false
            };

            match (score_is_cutoff, score, &entry.best_move) {
                // Best case: we have a cutoff score
                (true, Some(s), _) => {
                    self.hits += 1;
                    ProbeResult::Score(s)
                }
                // Useful case: we have a move for ordering (but no usable score)
                (false, _, Some(mv)) => {
                    self.hits += 1;
                    ProbeResult::Move(mv.clone())
                }
                // No useful information
                _ => {
                    self.misses += 1;
                    ProbeResult::Miss
                }
            }
        } else {
            self.misses += 1;
            ProbeResult::Miss
        }
    }

    /// Store an entry in the transposition table
    ///
    /// # Parameters
    /// * `hash` - Zobrist hash of the position
    /// * `best_move` - Best move found
    /// * `score` - Search score
    /// * `depth` - Remaining depth that was searched
    /// * `ply` - Distance from root (for mate score adjustment)
    /// * `bound_type` - Type of bound
    pub fn store(
        &mut self,
        hash: u64,
        best_move: Option<PlayerMove>,
        score: f32,
        depth: usize,
        ply: usize,
        bound_type: BoundType,
    ) {
        // Improved replacement logic considering both depth and ply
        if let Some(existing) = self.table.get(&hash) {
            // Keep deeper searches, but also prefer entries from closer to root for same depth
            let should_keep_existing = existing.depth > depth
                || (existing.depth == depth
                    && existing.ply < ply
                    && existing.age >= self.current_age.saturating_sub(2));

            if should_keep_existing {
                return;
            }
        }

        // Adjust mate scores for storage
        let adjusted_score = self.adjust_mate_score_for_tt(score, ply);

        let entry = TTEntry::new(
            hash,
            best_move,
            adjusted_score,
            depth,
            ply,
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
        if score.abs() > values::MATE_THRESHOLD {
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
        if score.abs() > values::MATE_THRESHOLD {
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
        // Improved replacement scheme considering depth, ply, and age
        if let Some((&worst_hash, _)) = self
            .table
            .iter()
            .min_by_key(|(_, entry)| (entry.age, entry.depth, std::cmp::Reverse(entry.ply)))
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
