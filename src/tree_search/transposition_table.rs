use super::tree_node::{TreeNode, TreeNodeRef};
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc, rc::Weak};

pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct TTEntry {
    pub node: Weak<RefCell<TreeNode>>,
    pub depth: usize,
    pub flag: TTFlag,
    pub score: f32,
    pub generation: u8,
}

pub struct TranspositionTable {
    table: HashMap<u64, TTEntry>,
    current_generation : u8,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            table: HashMap::new(),
            current_generation : 0
        }
    }

    /// Start a new search generation
    pub fn new_search(&mut self) {
        self.current_generation = self.current_generation.wrapping_add(1);
    }

    /// Determine if we should replace an existing entry
    fn should_replace(&self, new_depth: usize, old_entry: &TTEntry) -> bool {
        old_entry.generation != self.current_generation || new_depth > old_entry.depth
    }

    /// Insert a new entry into the transposition table
    pub fn insert_entry(
        &mut self,
        hash: u64,
        node: TreeNodeRef,
        depth: usize,
        flag: TTFlag,
        score: f32,
    ) {
        // Check if we should replace existing entry
        if let Some(existing) = self.table.get(&hash) {
            if !self.should_replace(depth, existing) {
                return;
            }
        }

        let weak_node = Rc::downgrade(&node);
        self.table.insert(
            hash,
            TTEntry {
                node: weak_node,
                depth,
                flag,
                score,
                generation: self.current_generation,
            },
        );
    }

    /// Get an entry from its hash with more flexible depth handling
    pub fn get_entry(&mut self, hash: u64, depth: usize) -> Option<&TTEntry> {
        let entry_opt = self.table.get(&hash);
        if let Some(entry) = entry_opt {
            // Reject entries from old generations
            if entry.generation != self.current_generation {
                return None;
            }

            // Check if entry is useful for current search
            // Entry should be deep enough to be useful (you can adjust this threshold)
            if entry.depth >= depth {
                return Some(entry);
            }

            // Check if entry is useful for current search
            // Entry should be deep enough to be useful (you can adjust this threshold)
            if entry.depth >= depth {
                return Some(entry);
            }
        }
        None
    }

    pub fn get_old_entry_score(&self, hash: u64) -> Option<f32> {
        let entry_opt = self.table.get(&hash);
        if let Some(entry) = entry_opt {
            Some(entry.score)
        } else {
            None
        }
    }

    /// Remove all entries from the hash table
    pub fn clear(&mut self) {
        self.table.clear();
    }

    /// Clean old or invalid entries
    pub fn maintenance(&mut self) {
        self.table.retain(|_, entry| {
            // Keep entry if it's from current generation and node is still valid
            entry.generation == self.current_generation && entry.node.upgrade().is_some()
        });
    }
}