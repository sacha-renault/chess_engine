use derive_builder::Builder;

use super::super::evaluator_trait::Evaluator;
use super::utility::{classic_heuristic_move_bonus, get_value_by_piece, get_value_multiplier_by_piece};

use crate::pieces::static_positions::*;
use crate::pieces::moves;
use crate::boards::ColorBoard;
use crate::game_engine::engine::Engine;
use crate::pieces::{Color, Piece};
use crate::game_engine::player_move::PlayerMove;

// const values
const TOTAL_OPENING_MATERIAL: i32 = 78;
const INITIAL_MINORS: [u64; 2] = [ WHITE_BISHOPS | WHITE_KNIGHTS, BLACK_BISHOPS | BLACK_KNIGHTS ];
const CENTER_SQUARES: u64 = (RANK4 | RANK5) & (FILE_D | FILE_E);
const EXTENDED_CENTER: u64 = (RANK3 | RANK4 | RANK5 | RANK6) & (FILE_C | FILE_D | FILE_E | FILE_F);

#[derive(Builder, Default)]
pub struct AdvancedEvaluator {
    #[builder(default = "0.2")]
    weight_ignore_val: f32,

    #[builder(default = "2.0")]
    knight_develop_bonus: f32,

    #[builder(default = "2.0")]
    bishop_develop_bonus: f32,

    #[builder(default = "2.50")]
    queen_early_penalty: f32,

    #[builder(default = "3.0")]
    center_pawn_bonus: f32,

    #[builder(default = "1.5")]
    extended_center_pawn_bonus: f32,

    #[builder(default = "2.0")]
    center_minor_bonus: f32,

    #[builder(default = "1.0")]
    extended_center_minor_bonus: f32,

    #[builder(default = "1.5")]
    center_queen_bonus: f32,

    #[builder(default = "1.0")]
    center_control_bonus: f32,

    #[builder(default = "2.0")]
    pawn_shield_bonus: f32,

    #[builder(default = "15")]
    endgame_material_threshold: usize,

    #[builder(default = "2.0")]
    double_pawn_penalty: f32,

    #[builder(default = "2.0")]
    isoled_pawn_penalty: f32,

    #[builder(default = "2.0")]
    pawn_chain_bonus: f32,

    #[builder(default = "2.0")]
    king_center_bonus: f32,

    #[builder(default = "3.0")]
    passed_pawn_bonus: f32,

    #[builder(default = "1.0")]
    rank_multiplier: f32,
}

impl AdvancedEvaluator {
    /// represent how much
    /// we should weight every score calculation
    /// 1. represent full opening / mid game score calculation
    /// 0. represent full end game
    fn evaluate_game_state(&self, pieces: &Vec<(u64, Piece, Color)>) -> f32 {
        let mut total_material = 0;

        for (_, piece, _) in pieces {
            total_material += match piece {
                Piece::Queen => 9,
                Piece::Rook => 5,
                Piece::Bishop | Piece::Knight => 3,
                Piece::Pawn => 1,
                Piece::King => 0,
            };
        }

        // Scale from 1.0 (opening) to 0.0 (endgame)
        let phase = (total_material as f32 - self.endgame_material_threshold as f32) /
                   (TOTAL_OPENING_MATERIAL as f32 - self.endgame_material_threshold as f32);
        phase.max(0.0).min(1.0)
    }

    fn calculate_opening_score(&self, engine: &Engine, pieces: &Vec<(u64, Piece, Color)>) -> f32 {
        let board = engine.get_board();
        let white_board = &board.white;
        let black_board = &board.black;
        let white_score = self.calculate_opening_score_side(
            white_board,
            black_board,
            pieces,
            true);
        let black_score = self.calculate_opening_score_side(
            black_board,
            white_board,
            pieces,
            false);
        white_score - black_score
    }

    fn calculate_end_score(&self, engine: &Engine, pieces: &Vec<(u64, Piece, Color)>) -> f32 {
        let board = engine.get_board();
        let white_board = &board.white;
        let black_board = &board.black;
        let white_score = self.calculate_end_score_side(
            white_board,
            black_board,
            pieces,
            true);
        let black_score = self.calculate_end_score_side(
            black_board,
            white_board,
            pieces,
            false);
        white_score - black_score
    }

    fn calculate_opening_score_side(
        &self,
        player_board: &ColorBoard,
        opponent_board: &ColorBoard,
        pieces: &Vec<(u64, Piece, Color)>,
        is_white: bool
    ) -> f32 {
        let mut score = 0.0;

        // Material score
        score += self.evaluate_material(pieces);

        // Piece development
        score += self.evaluate_development_side(player_board, is_white);

        // Center control
        score += self.evaluate_center_occupation(player_board);

        // Center attacks
        score += self.evaluate_center_attacks(player_board, opponent_board, is_white);

        // King safety
        score += self.evaluate_king_safety_side(player_board.king, player_board.pawn);

        // Pawn structure
        score += self.evaluate_pawn_structure_side(player_board.pawn, is_white);

        score
    }

    fn calculate_end_score_side(
        &self,
        player_board: &ColorBoard,
        opponent_board: &ColorBoard,
        pieces: &Vec<(u64, Piece, Color)>,
        is_white: bool
    ) -> f32 {
        let mut score = 0.0;

        // Material score (weighted differently in endgame)
        score += self.evaluate_material(pieces) * 1.5;

        // // King centralization
        score += self.calculate_king_center_bonus(player_board.king.trailing_zeros() as u8);

        // // Passed pawns (more important in endgame)
        score += self.evaluate_passed_pawns_side(player_board.pawn, opponent_board.pawn, is_white);

        // // King safety (less important in endgame)
        score += self.evaluate_king_safety_side(player_board.king, player_board.pawn) * 0.5;

        // Pawn structure (less important in endgame)
        score += self.evaluate_pawn_structure_side(player_board.pawn, is_white) * 0.5;

        score
    }

    // Helper evaluation functions
    fn evaluate_material(&self, pieces: &Vec<(u64, Piece, Color)>) -> f32 {
        let mut score = 0.0;
        for (bitboard, piece, color) in pieces {
            let piece_value = get_value_by_piece(*piece);
            let position_multiplier = get_value_multiplier_by_piece(*piece, *color, *bitboard);
            score += piece_value * position_multiplier;
        }
        score
    }

    fn clip_weight_values(&self, weight: f32) -> (f32, f32) {
        if weight < self.weight_ignore_val {
            (0. , 1.)
        } else if 1. - weight > 1. - self.weight_ignore_val {
            (1., 0.)
        } else {
            (weight, 1. - weight)
        }
    }

    fn evaluate_development_side(&self, player_board: &ColorBoard, is_white: bool) -> f32 {
        let mut score = 0.0;
        let side_idx = if is_white { 0 } else { 1 };

        // Check knight development
        let undeveloped_knights = player_board.knight & if is_white { WHITE_KNIGHTS } else { BLACK_KNIGHTS};
        score -= (undeveloped_knights.count_ones() as f32) * self.knight_develop_bonus;

        // Check bishop development
        let undeveloped_bishops = player_board.bishop & if is_white { WHITE_BISHOPS } else { BLACK_BISHOPS };
        score -= (undeveloped_bishops.count_ones() as f32) * self.bishop_develop_bonus;

        // Penalize early queen development (if queen has moved but minor pieces haven't)
        if (player_board.queen != 0) &&
           (player_board.queen != if is_white { WHITE_QUEEN } else { BLACK_QUEEN }) {
            let undeveloped_minors = (player_board.knight | player_board.bishop) & INITIAL_MINORS[side_idx];
            if undeveloped_minors != 0 {
                score -= self.queen_early_penalty;
            }
        }

        score
    }

    fn evaluate_center_occupation(&self, player_board: &ColorBoard) -> f32 {
        let mut score = 0.0;

        // Pawns in center
        let center_pawns = player_board.pawn & CENTER_SQUARES;
        score += (center_pawns.count_ones() as f32) * self.center_pawn_bonus;

        let extended_pawns = player_board.pawn & EXTENDED_CENTER;
        score += (extended_pawns.count_ones() as f32) * self.extended_center_pawn_bonus;

        // Minor pieces in center
        let center_minors = (player_board.knight | player_board.bishop) & CENTER_SQUARES;
        score += (center_minors.count_ones() as f32) * self.center_minor_bonus;

        let extended_minors = (player_board.knight | player_board.bishop) & EXTENDED_CENTER;
        score += (extended_minors.count_ones() as f32) * self.extended_center_minor_bonus;

        // Queen influence in center
        let center_queen = player_board.queen & (CENTER_SQUARES | EXTENDED_CENTER);
        score += (center_queen.count_ones() as f32) * self.center_queen_bonus;

        score
    }

    fn evaluate_center_attacks(&self, player_board: &ColorBoard, opponent_board: &ColorBoard, is_white: bool) -> f32 {
        let mut score = 0.0;

        // Get attacked squares in center
        let center_attacks = self.get_attacked_squares(player_board, opponent_board, is_white) &
            (CENTER_SQUARES | EXTENDED_CENTER);

        // Bonus for each central square attacked
        score += (center_attacks & CENTER_SQUARES).count_ones() as f32 * self.center_control_bonus * 2.0;
        score += (center_attacks & EXTENDED_CENTER).count_ones() as f32 * self.center_control_bonus;

        score
    }

    fn get_attacked_squares(&self, color_board: &ColorBoard, opponent_board: &ColorBoard, is_white: bool) -> u64 {
        let mut attacked = 0u64;
        let same_color_bitboard = color_board.bitboard();
        let other_color_bitboard = opponent_board.bitboard();
        let color = if is_white { Color::White } else { Color::Black };

        // Pawn attacks
        attacked |= moves::pawn_captures(color_board.pawn, other_color_bitboard, color);

        // Knight attacks
        attacked |= moves::knight_moves(color_board.knight, same_color_bitboard);

        // Bishop/Queen diagonal attacks
        let diagonal_sliders = color_board.bishop | color_board.queen;
        attacked |= moves::bishops_moves(diagonal_sliders, same_color_bitboard, other_color_bitboard);

        // Rook/Queen straight attacks
        let straight_sliders = color_board.rook | color_board.queen;
        attacked |= moves::rooks_moves(straight_sliders, same_color_bitboard, other_color_bitboard);

        attacked
    }

    fn evaluate_king_safety_side(
        &self,
        king: u64,
        pawn: u64,
    ) -> f32 {
        // get all square the king legally can go
        let king_legal_moves = moves::king_moves(king, 0u64);
        let pawn_shield = king_legal_moves & pawn;
        (pawn_shield.count_ones() as f32) * self.pawn_shield_bonus
    }

    fn calculate_king_center_bonus(&self, king_square: u8) -> f32 {
        let file = king_square % 8;
        let rank = king_square / 8;

        // Calculate distance from center (both file and rank)
        let file_center_distance = (3.5 - file as f32).abs();
        let rank_center_distance = (3.5 - rank as f32).abs();

        // The closer to the center, the higher the bonus
        let center_bonus = (8.0 - file_center_distance - rank_center_distance) * self.king_center_bonus;
        center_bonus
    }

    fn evaluate_pawn_structure_side(&self, pawn: u64, is_white: bool) -> f32 {
        let mut score = 0.0;

        // Check for doubled pawns
        for file in 0..8 {
            let file_mask = FILE_A << file;
            let pawns_in_file = pawn & file_mask;
            if pawns_in_file.count_ones() > 1 {
                score -= self.double_pawn_penalty;
            }
        }

        // Check for isolated pawns
        for file in 0..8 {
            let file_mask = FILE_A << file;
            let adjacent_files = if file == 0 {
                FILE_B
            } else if file == 7 {
                FILE_G
            } else {
                (FILE_A << (file - 1)) | (FILE_A << (file + 1))
            };

            let pawns_in_file = pawn & file_mask;
            let pawns_in_adjacent = pawn & adjacent_files;

            if pawns_in_file != 0 && pawns_in_adjacent == 0 {
                score += self.isoled_pawn_penalty;
            }
        }

        // Check for backward pawns and pawn chains
        let pawn_chain = if is_white {
            moves::pawn_captures(pawn, pawn, Color::White)
        } else {
            moves::pawn_captures(pawn, pawn, Color::Black)
        };

        // Bonus for pawn chains
        let supported_pawns = pawn & pawn_chain;
        score += (supported_pawns.count_ones() as f32) * self.pawn_chain_bonus;

        score
    }

    fn evaluate_passed_pawns_side(&self, mut pawn: u64, enemy_pawn: u64, is_white: bool) -> f32 {
        let mut score = 0.0;

        // For each pawn, check if it's passed
        while pawn != 0 {
            let pawn_square = pawn.trailing_zeros() as u8;
            let file = pawn_square % 8;
            let rank = if is_white { pawn_square / 8 } else { 7 - (pawn_square / 8) };

            // Create mask for squares in front of pawn
            let passed_mask = if is_white {
                !((1u64 << pawn_square) - 1)
            } else {
                (1u64 << pawn_square) - 1
            };

            // Check adjacent files for enemy pawns
            let file_mask = if file == 0 {
                FILE_A | FILE_B
            } else if file == 7 {
                FILE_G | FILE_H
            } else {
                (FILE_A << (file - 1)) | (FILE_A << file) | (FILE_A << (file + 1))
            };

            let blocking_pawns = enemy_pawn & file_mask & passed_mask;

            // If no blocking pawns, this is a passed pawn
            if blocking_pawns == 0 {
                // Base bonus plus extra for advancement
                score += self.passed_pawn_bonus + (rank as f32 * self.rank_multiplier);
            }

            // Clear the least significant bit
            pawn &= pawn - 1;
        }

        score
    }
}

impl Evaluator for AdvancedEvaluator {
    fn evaluate_engine_state(&self, engine: &Engine, _: usize) -> f32 {
        let board = engine.get_board();
        let pieces = board.individual_pieces();
        let weight = self.evaluate_game_state(&pieces);

        let (opening_weight, end_weight) = self.clip_weight_values(weight);
        println!("Weight {} : {}:{}", weight, opening_weight, end_weight);

        let mut score = 0.0;

        if opening_weight != 0.0 {
            score += opening_weight * self.calculate_opening_score(engine, &pieces);
        }
        if end_weight != 0.0 {
            score += end_weight * self.calculate_end_score(engine, &pieces);
        }

        score
    }

    fn evaluate_heuristic_move(
        &self,
        player_move: PlayerMove,
        moved_piece: Piece,
        captured_piece_opt: Option<Piece>,
        is_king_checked: bool
    ) -> f32 {
        classic_heuristic_move_bonus(player_move, moved_piece, captured_piece_opt, is_king_checked)
    }
}