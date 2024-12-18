use super::board::Board;
use super::color::Color;
use super::color_board::ColorBoard;
use super::pieces::Pieces;
use super::utility::{
    coordinates_to_u64, get_color, get_piece_type, get_possible_move, is_king_checked, move_piece,
};

pub struct Engine {
    // rules
    board: Board,
    white_turn: bool,
    // en_passant: Option<usize>, // Optional field for en passant target square
    // castling_rights: (bool, bool, bool, bool), // (white_king_side, white_queen_side, black_king_side, black_queen_side)
    halfmove_clock: u32, // Number of halfmoves since the last pawn move or capture
    fullmove_number: u32, // Number of full moves in the game
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::new(),
            white_turn: true,
            // en_passant: None,
            // castling_rights: (true, true, true, true),
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }

    /// Main play function
    pub fn play(&mut self, current: (usize, usize), target: (usize, usize)) -> Result<(), String> {
        let current_square = coordinates_to_u64(current);
        let target_square = coordinates_to_u64(target);

        // Validate the move and get the new board state
        let new_board = self.validate_move(current_square, target_square)?;

        // Apply the new board state
        self.board = new_board;

        // Finalize the turn
        self.finalize_turn();

        Ok(())
    }

    /// Validate the move before overwrite board state
    fn validate_move(&self, current_square: u64, target_square: u64) -> Result<Board, String> {
        let (player_board, opponent_board) = self.get_half_turn_boards();
        let piece_type = get_piece_type(player_board, current_square);

        // Ensure there is a piece at the current square
        if piece_type.is_none() {
            return Err("No piece at this location".to_string());
        }

        let piece = piece_type.unwrap();
        let color = get_color(self.white_turn);

        // Get the possible moves for the piece
        let possible_moves = get_possible_move(
            &piece,
            current_square,
            player_board.bitboard(),
            opponent_board.bitboard(),
            &color,
        );

        // Check if the target square is a valid move
        if target_square & possible_moves == 0 {
            return Err("This isn't a possible move".to_string());
        }

        // Simulate the move and check if the king is in check
        let simulated_board =
            self.simulate_and_check_move(current_square, target_square, &piece, &color)?;

        Ok(simulated_board)
    }

    /// Simulate and check if the king is in check
    fn simulate_and_check_move(
        &self,
        current_square: u64,
        target_square: u64,
        piece: &Pieces,
        color: &Color,
    ) -> Result<Board, String> {
        // Simulate the move
        let simulated_board = move_piece(
            self.board.clone(),
            current_square,
            target_square,
            color,
            piece,
        );

        // Get the simulated player's and opponent's boards
        let (player_board, opponent_board) = if *color == Color::White {
            (&simulated_board.white, &simulated_board.black)
        } else {
            (&simulated_board.black, &simulated_board.white)
        };

        // Check if the king is in check in the simulated state
        // For that, we check all possible moves for next round (bulk computed all opponent moves)
        // and check if kinng_bitbord & all_moves == 0 => no check
        if is_king_checked(player_board.king, opponent_board, player_board, color) {
            return Err("Move leaves the king in check".to_string());
        }

        Ok(simulated_board)
    }

    /// Finalize the turn after a move
    fn finalize_turn(&mut self) {
        self.halfmove_clock += 1;
        if !self.white_turn {
            self.fullmove_number += 1;
        }
        self.white_turn = !self.white_turn;
    }

    fn get_half_turn_boards(&self) -> (&ColorBoard, &ColorBoard) {
        let board = if self.white_turn {
            &self.board.white
        } else {
            &self.board.black
        };
        let opponent_board = if self.white_turn {
            &self.board.black
        } else {
            &self.board.white
        };
        (board, opponent_board)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
}
