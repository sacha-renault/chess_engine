use super::evaluate::Evaluator;
use super::tree::Tree;
use super::values;

use crate::game_engine::engine::Engine;

pub struct TreeBuilder {
    evaluator: Option<Box<dyn Evaluator>>,
    max_depth: Option<usize>,
    max_size: Option<usize>,
    foreseeing_windowing: Option<f32>,
}

impl TreeBuilder {
    pub fn new() -> Self {
        Self {
            evaluator: None,
            max_depth: None,
            max_size: None,
            foreseeing_windowing: None,
        }
    }

    pub fn evaluator(mut self, evaluator: Box<dyn Evaluator>) -> Self {
        self.evaluator = Some(evaluator);
        self
    }

    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    pub fn max_size(mut self, size: usize) -> Self {
        self.max_size = Some(size);
        self
    }

    pub fn foreseeing_windowing(mut self, window: f32) -> Self {
        self.foreseeing_windowing = Some(window);
        self
    }

    pub fn build_tree(self, engine: Engine) -> Result<Tree, ()> {
        match (self.max_depth, self.max_size) {
            // Cannot start a tree with both size and max depth unset
            // It would result in an infinit tree that would never be able to compute
            // Any result
            (None, None) => return Err(()),
            _ => {}
        };

        let tree = Tree::new(
            engine,
            self.evaluator.expect("Evaluator must be set"),
            self.max_depth.unwrap_or(usize::MAX),
            self.max_size.unwrap_or(usize::MAX),
            self.foreseeing_windowing
                .unwrap_or(values::FORESEEING_WINDOW),
        );

        Ok(tree)
    }
}
