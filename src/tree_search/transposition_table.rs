use super::tree_node::{TreeNode, TreeNodeRef};
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc, rc::Weak};

pub enum TTFlag {
    Exact,
    LowerBound,
    UperBound,
}

pub struct TTEntry {
    pub node: Weak<RefCell<TreeNode>>,
    pub depth: usize,
    pub flag: TTFlag,
    pub score: f32,
}

pub struct TranspositionTable(HashMap<u64, TTEntry>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(HashMap::new())
    }

    /// Insert a new entry into the transposition table
    ///
    /// # Arguments
    /// * `hash` - The Zobrist hash of the position
    /// * `node` - A strong reference to the TreeNode
    pub fn insert_entry(&mut self, hash: u64, node: TreeNodeRef, depth: usize, flag: TTFlag, score: f32) {
        // Create a Weak reference to the TreeNode
        let weak_node = Rc::downgrade(&node);

        // Insert the new entry into the HashMap
        self.0.insert(
            hash,
            TTEntry {
                node: weak_node,
                depth,
                flag,
                score
            },
        );
    }

    /// Get an entry from its hash, ensure the week pointer is pointing on
    /// some actual data. Otherwise, return none and remove the entry
    pub fn get_entry(&mut self, hash: u64, depth: usize) -> Option<&TTEntry> {
        // Get the entry if it exists
        let entry_opt = self.0.get(&hash);
        if let Some(entry) = entry_opt {
            // Check if depth is same
            if entry.depth != depth {
                return None;
            }

            // Check if the weak reference is still valid
            if entry.node.upgrade().is_some() {
                return self.0.get(&hash);
            } else {
                self.0.remove(&hash);
            }
        }
        None
    }

    /// Remove all entries from the hash table
    pub fn clear(&mut self) {
        self.0.clear();
    }
}
