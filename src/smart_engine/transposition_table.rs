use super::tree_node::{TreeNode, TreeNodeRef};
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc, rc::Weak};

pub struct TranspositionTable(HashMap<u64, Weak<RefCell<TreeNode>>>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(HashMap::new())
    }

    /// Insert a new entry into the transposition table
    ///
    /// # Arguments
    /// * `hash` - The Zobrist hash of the position
    /// * `node` - A strong reference to the TreeNode
    pub fn insert_entry(&mut self, hash: u64, node: TreeNodeRef) {
        // Create a Weak reference to the TreeNode
        let weak_node = Rc::downgrade(&node);

        // Insert the new entry into the HashMap
        self.0.insert(hash, weak_node);
    }

    /// Get an entry from its hash, ensure the week pointer is pointing on
    /// some actual data. Otherwise, return none and remove the entry
    pub fn get_entry(&mut self, hash: u64) -> Option<Rc<RefCell<TreeNode>>> {
        // Get the entry if it exists
        if let Some(weak_ref) = self.0.get(&hash) {
            // Check if the weak reference is still valid
            let upgraded = weak_ref.upgrade();
            if let Some(data) = upgraded {
                return Some(data.clone());
            } else {
                self.0.remove(&hash);
            }
        }
        None
    }
}
