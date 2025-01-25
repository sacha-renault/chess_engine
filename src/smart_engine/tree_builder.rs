use super::evaluate::Evaluator;
use super::tree::Tree;
use super::values;

use crate::game_engine::engine::Engine;

/// A builder for constructing game analysis trees with customizable parameters
///
/// The TreeBuilder uses the builder pattern to configure and create a game analysis tree
/// with specific constraints and evaluation strategies.
pub struct TreeBuilder {
    max_depth: Option<usize>,
    max_size: Option<usize>,
    foreseeing_windowing: Option<f32>,
}

impl TreeBuilder {
    /// Creates a new TreeBuilder with default settings (all parameters unset)
    pub fn new() -> Self {
        Self {
            max_depth: None,
            max_size: None,
            foreseeing_windowing: None,
        }
    }

    /// Sets the maximum depth the tree will explore
    ///
    /// # Arguments
    /// * `depth` - Maximum number of plies (half-moves) to analyze
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Sets the maximum number of positions to store in the tree
    ///
    /// # Arguments
    /// * `size` - Maximum number of nodes in the tree
    pub fn max_size(mut self, size: usize) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Sets the foreseeing window for position evaluation
    ///
    /// # Arguments
    /// * `window` - A float value representing the evaluation window size
    pub fn foreseeing_windowing(mut self, window: f32) -> Self {
        self.foreseeing_windowing = Some(window);
        self
    }

    /// Builds and returns a new game analysis tree with the configured parameters
    ///
    /// # Arguments
    /// * `engine` - The initial game engine state to analyze
    ///
    /// # Returns
    /// * `Ok(Tree)` - A newly constructed analysis tree
    /// * `Err(())` - If both max_depth and max_size are None
    ///
    /// # Panics
    /// * If no evaluator was set
    pub fn build_tree(self, engine: Engine, evaluator: Box<dyn Evaluator>) -> Result<Tree, ()> {
        match (self.max_depth, self.max_size) {
            // Cannot start a tree with both size and max depth unset
            // It would result in an infinit tree that would never be able to compute
            // Any result
            (None, None) => return Err(()),
            _ => {}
        };

        let tree = Tree::new(
            engine,
            evaluator,
            self.max_depth.unwrap_or(usize::MAX),
            self.max_size.unwrap_or(usize::MAX),
            self.foreseeing_windowing
                .unwrap_or(values::FORESEEING_WINDOW),
        );

        Ok(tree)
    }
}
