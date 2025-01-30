use crate::game_engine::player_move::PlayerMove;

pub struct DbRatios {
    pub white_win_rate: f32,
    pub draws_rate: f32,
    pub black_win_rate: f32
}

impl DbRatios {
    pub fn to_eval(&self) -> f32 {
        // Draws are weighted at 0.5 for both sides
        let white_score = self.white_win_rate + (self.draws_rate * 0.5);
        let black_score = self.black_win_rate + (self.draws_rate * 0.5);
        
        // Convert to centipawn-like scale (multiply by 100)
        // This makes it more comparable with typical engine evaluations
        (white_score - black_score) * 100.0
    }
}

pub struct TreeEval {
    pub score: f32,
    pub depth: usize,
    pub mate_depth: Option<usize>
}

pub enum MoveEvaluation {
    TreeEvaluation(TreeEval), DbRating(DbRatios)
}

impl MoveEvaluation {
    // Helper to get a comparable score regardless of source
    pub fn to_score(&self) -> f32 {
        match self {
            MoveEvaluation::TreeEvaluation(tree_eval) => tree_eval.score,
            MoveEvaluation::DbRating(ratios) => ratios.to_eval(),
        }
    }
}

pub struct NextMove {
    pub chess_move: PlayerMove,
    pub eval: MoveEvaluation
}

impl NextMove {
    pub fn new_from_tree(chess_move: PlayerMove, score: f32, depth: usize, mate_depth: Option<usize>) -> Self {
        Self {
            chess_move,
            eval: MoveEvaluation::TreeEvaluation(TreeEval { score, depth, mate_depth })
        }
    }

    pub fn new_from_db(chess_move: PlayerMove, white_win_rate: f32, draws_rate: f32, black_win_rate: f32) -> Self {
        Self {
            chess_move,
            eval: MoveEvaluation::DbRating(DbRatios { white_win_rate, draws_rate, black_win_rate }),
        }
    }
}

impl PartialEq for NextMove {
    fn eq(&self, other: &Self) -> bool {
        self.eval.to_score() == other.eval.to_score()
    }
}

impl PartialOrd for NextMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.eval.to_score().partial_cmp(&other.eval.to_score())
    }
}