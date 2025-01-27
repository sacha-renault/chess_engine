use crate::boards::board::Board;

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> f32;
}
