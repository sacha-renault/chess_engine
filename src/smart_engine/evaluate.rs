use crate::boards::board::Board;

pub trait Evaluator {
    fn evaluate(board: &Board) -> f32;
}
