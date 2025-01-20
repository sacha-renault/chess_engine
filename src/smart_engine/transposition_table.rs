use super::tree_node::{TreeNode, TreeNodeRef};
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc, rc::Weak};

struct TTEntry {
    node: Weak<RefCell<TreeNode>>,
    depth: usize,
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
    pub fn insert_entry(&mut self, hash: u64, node: TreeNodeRef, depth: usize) {
        // Create a Weak reference to the TreeNode
        let weak_node = Rc::downgrade(&node);

        // Insert the new entry into the HashMap
        self.0.insert(
            hash,
            TTEntry {
                node: weak_node,
                depth,
            },
        );
    }

    /// Get an entry from its hash, ensure the week pointer is pointing on
    /// some actual data. Otherwise, return none and remove the entry
    pub fn get_entry(&mut self, hash: u64, depth: usize) -> Option<Rc<RefCell<TreeNode>>> {
        // Get the entry if it exists
        if let Some(entry) = self.0.get(&hash) {
            // Check if depth is same
            if entry.depth != depth {
                return None;
            }

            // Check if the weak reference is still valid
            let upgraded = entry.node.upgrade();
            if let Some(data) = upgraded {
                return Some(data.clone());
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
