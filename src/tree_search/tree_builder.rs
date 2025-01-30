use super::tree::Tree;

use crate::static_evaluation::values;
use crate::static_evaluation::evaluator_trait::Evaluator;
use crate::static_evaluation::evaluators::BasicEvaluator;
use crate::game_engine::engine::Engine;

/// A builder for constructing game analysis trees with customizable parameters
///
/// The TreeBuilder uses the builder pattern to configure and create a game analysis tree
/// with specific constraints and evaluation strategies.
pub struct TreeBuilder {
    max_depth: Option<usize>,
    max_size: Option<usize>,
    max_q_depth: Option<usize>,
    razoring_margin_base: Option<f32>,
    razoring_depth: Option<usize>,
    engine: Option<Engine>,
    evaluator: Option<Box<dyn Evaluator>>
}

impl TreeBuilder {
    /// Creates a new TreeBuilder with default settings (all parameters unset)
    pub fn new() -> Self {
        Self {
            max_depth: None,
            max_size: None,
            max_q_depth: None,
            razoring_margin_base: None,
            razoring_depth: None,
            engine: None,
            evaluator: None
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

    /// Sets the razoring depth the tree will explore
    ///
    /// # Arguments
    /// * `depth` - Min number of plies (half-move) before razoring
    pub fn razoring_depth(mut self, depth: usize) -> Self {
        self.razoring_depth = Some(depth);
        self
    }

    /// Set the razoring
    ///
    /// # Arguments
    /// * `depth` - Min number of plies (half-move) before razoring
    pub fn razoring_margin_base(mut self, value: f32) -> Self {
        self.razoring_margin_base = Some(value);
        self
    }

    /// Sets the maximum depth the quiescence search will explore
    ///
    /// # Arguments
    /// * `depth` - Maximum number of plies (half-moves) to analyze
    pub fn max_quiescence_depth(mut self, depth: usize) -> Self {
        self.max_q_depth = Some(depth);
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

    /// Sets the engine for the tree
    ///
    /// # Arguments
    /// * `engine` - Engine
    pub fn engine(mut self, engine: Engine) -> Self {
        self.engine = Some(engine);
        self
    }

    /// Set the evaluator for the tree
    ///
    /// # Arguments
    /// * `evaluator` - struct that impl Evaluator trait
    pub fn evaluator(mut self, evaluator: Box<dyn Evaluator>) -> Self {
        self.evaluator = Some(evaluator);
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
    pub fn build_tree(self) -> Result<Tree, ()> {
        match (self.max_depth, self.max_size) {
            // Cannot start a tree with both size and max depth unset
            // It would result in an infinit tree that would never be able to compute
            // Any result
            (None, None) => return Err(()),
            _ => {}
        };

        let tree = Tree::new(
            self.engine.unwrap_or(Engine::new()),
            self.evaluator.unwrap_or(Box::new(BasicEvaluator::new())),
            self.max_depth.unwrap_or(usize::MAX),
            self.max_size.unwrap_or(usize::MAX),
            self.max_q_depth.unwrap_or(usize::MAX),
            self.razoring_margin_base.unwrap_or(values::RAZORING_MARGIN_BASE),
            self.razoring_depth.unwrap_or(values::RAZORING_DEPTH),
        );

        Ok(tree)
    }
}
