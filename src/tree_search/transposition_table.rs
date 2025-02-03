use super::tree_node::{TreeNode, TreeNodeRef};
use std::cell::RefCell;
use std::sync::RwLock;
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
    table: RwLock<HashMap<u64, TTEntry>>,
    current_generation: RwLock<u8>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable {
            table: RwLock::new(HashMap::new()),
            current_generation: RwLock::new(0),
        }
    }

    /// Start a new search generation
    pub fn new_search(&self) {
        let mut gen = self.current_generation.write().unwrap();
        *gen = gen.wrapping_add(1);
    }

    /// Determine if we should replace an existing entry
    fn should_replace(&self, new_depth: usize, old_entry: &TTEntry) -> bool {
        let current_gen = *self.current_generation.read().unwrap();
        old_entry.generation != current_gen || new_depth > old_entry.depth
    }

    /// Insert a new entry into the transposition table
    pub fn insert_entry(
        &self,
        hash: u64,
        node: TreeNodeRef,
        depth: usize,
        flag: TTFlag,
        score: f32,
    ) {
        let mut table = self.table.write().unwrap();

        // Check if we should replace existing entry
        if let Some(existing) = table.get(&hash) {
            if !self.should_replace(depth, existing) {
                return;
            }
        }

        let weak_node = Rc::downgrade(&node);
        let current_gen = *self.current_generation.read().unwrap();

        table.insert(
            hash,
            TTEntry {
                node: weak_node,
                depth,
                flag,
                score,
                generation: current_gen,
            },
        );
    }

    /// Get an entry from its hash with more flexible depth handling
    pub fn get_entry(&self, hash: u64, depth: usize) -> Option<TTEntry> {
        let table = self.table.read().unwrap();
        let current_gen = *self.current_generation.read().unwrap();

        if let Some(entry) = table.get(&hash) {
            // Reject entries from old generations
            if entry.generation != current_gen {
                return None;
            }

            // Check if entry is useful for current search
            if entry.depth >= depth {
                return Some(entry.clone());
            }
        }
        None
    }

    pub fn get_old_entry_score(&self, hash: u64) -> Option<f32> {
        let table = self.table.read().unwrap();
        table.get(&hash).map(|entry| entry.score)
    }

    /// Remove all entries from the hash table
    pub fn clear(&self) {
        let mut table = self.table.write().unwrap();
        table.clear();
    }

    /// Clean old or invalid entries
    pub fn maintenance(&self) {
        let mut table = self.table.write().unwrap();
        let current_gen = *self.current_generation.read().unwrap();

        table.retain(|_, entry| {
            entry.generation == current_gen && entry.node.upgrade().is_some()
        });
    }
}

// Make TTEntry cloneable for thread-safe access
impl Clone for TTEntry {
    fn clone(&self) -> Self {
        TTEntry {
            node: self.node.clone(),
            depth: self.depth,
            flag: match self.flag {
                TTFlag::Exact => TTFlag::Exact,
                TTFlag::LowerBound => TTFlag::LowerBound,
                TTFlag::UpperBound => TTFlag::UpperBound,
            },
            score: self.score,
            generation: self.generation,
        }
    }
}