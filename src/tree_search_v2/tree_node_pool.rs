use crate::pieces::Piece;
use crate::prelude::{Engine, PlayerMove};

use super::tree_node::{NodeHandle, TreeNode};

pub struct TreeNodePool {
    nodes: Vec<Option<TreeNode>>,
    free_indices: Vec<usize>,
    capacity: usize,
    allocated_count: usize,
}

impl Default for TreeNodePool {
    fn default() -> Self {
        Self::with_capacity(1)
    }
}

impl TreeNodePool {
    /// Create a new pool with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        nodes.resize_with(capacity, || None);

        let free_indices = (0..capacity).rev().collect();

        TreeNodePool {
            nodes,
            free_indices,
            capacity,
            allocated_count: 0,
        }
    }

    /// Allocate a new node in the pool, returns None if pool is full
    pub fn allocate_node(
        &mut self,
        engine: Engine,
        score: f32,
        chess_move: Option<PlayerMove>,
        moved_piece: Option<Piece>,
        captured_piece: Option<Piece>,
    ) -> Option<NodeHandle> {
        if let Some(index) = self.free_indices.pop() {
            let node = TreeNode::new(engine, score, chess_move, moved_piece, captured_piece);
            self.nodes[index] = Some(node);
            self.allocated_count += 1;
            Some(NodeHandle(index))
        } else {
            None // Pool is full
        }
    }

    /// Get an immutable reference to a node by handle
    pub fn get_node(&self, handle: NodeHandle) -> Option<&TreeNode> {
        self.nodes.get(handle.0)?.as_ref()
    }

    /// Get a mutable reference to a node by handle
    pub fn get_node_mut(&mut self, handle: NodeHandle) -> Option<&mut TreeNode> {
        self.nodes.get_mut(handle.0)?.as_mut()
    }

    /// Free a node (returns it to the free pool)
    pub fn free_node(&mut self, handle: NodeHandle) -> bool {
        if handle.0 < self.capacity && self.nodes[handle.0].is_some() {
            self.nodes[handle.0] = None;
            self.free_indices.push(handle.0);
            self.allocated_count -= 1;
            true
        } else {
            false
        }
    }

    /// Clear all nodes (reset pool for reuse)
    pub fn clear(&mut self) {
        for node in &mut self.nodes {
            *node = None;
        }
        self.free_indices.clear();
        self.free_indices.extend((0..self.capacity).rev());
        self.allocated_count = 0;
    }

    /// Get the number of currently allocated nodes
    pub fn len(&self) -> usize {
        self.allocated_count
    }

    /// Check if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.allocated_count == 0
    }

    /// Get the total capacity of the pool
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the number of free slots remaining
    pub fn free_slots(&self) -> usize {
        self.capacity - self.allocated_count
    }

    /// Check if the pool is full
    pub fn is_full(&self) -> bool {
        self.allocated_count == self.capacity
    }
}
