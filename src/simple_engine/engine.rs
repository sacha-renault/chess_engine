use super::board::Board;
use super::color::Color;
use super::color_board::ColorBoard;
use super::pieces::Pieces;
use super::utility::{
    coordinates_to_u64, get_color, get_piece_type, get_possible_move, is_king_checked, move_piece,
};

/// Represents the state of the chess engine.
///
/// Fields:
///
/// - `board`: The current state of the chess board.
/// - `white_turn`: A boolean indicating if it's white's turn to move.
/// - `halfmove_clock`: The number of halfmoves since the last pawn move or capture.
pub struct Engine {
    // rules
    board: Board,
    white_turn: bool,
    // en_passant: Option<usize>, // Optional field for en passant target square
    // castling_rights: (bool, bool, bool, bool), // (white_king_side, white_queen_side, black_king_side, black_queen_side)
    halfmove_clock: u32, // Number of halfmoves since the last pawn move or capture
}

impl Engine {
    /// Creates a new instance of the `Engine` struct with the initial board setup.
    ///
    /// # Returns
    ///
    /// A new `Engine` instance with the following initial state:
    /// - `board`: A new `Board` instance representing the initial chessboard setup.
    /// - `white_turn`: A boolean set to `true`, indicating that it is White's turn to move.
    /// - `halfmove_clock`: An integer set to `0`, representing the number of half-moves since the last capture or pawn advance.
    ///
    /// # Example
    ///
    /// ```
    /// let engine = Engine::new();
    /// ```
    pub fn new() -> Self {
        Engine {
            board: Board::new(),
            white_turn: true,
            // en_passant: None,
            // castling_rights: (true, true, true, true),
            halfmove_clock: 0,
        }
    }

    /// Executes a move from the current position to the target position.
    ///
    /// # Arguments
    ///
    /// * `current` - A tuple representing the coordinates (row, column) of the piece to be moved.
    /// * `target` - A tuple representing the coordinates (row, column) of the target position.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move is valid and successfully executed.
    /// * `Err(String)` if the move is invalid, with an error message describing the reason.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * There is no piece at the current position.
    /// * The target position is not a valid move for the piece.
    /// * The move leaves the king in check.
    ///
    /// # Example
    ///
    /// ```
    /// let mut engine = Engine::new();
    /// engine.play((6, 4), (4, 4)).unwrap();
    /// ```
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
    ///
    /// # Arguments
    ///
    /// * `current_square` - The current position of the piece as a bitboard.
    /// * `target_square` - The target position of the piece as a bitboard.
    ///
    /// # Returns
    ///
    /// * `Ok(Board)` if the move is valid and the new board state.
    /// * `Err(String)` if the move is invalid, with an error message describing the reason.
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
    ///
    /// # Arguments
    ///
    /// * `current_square` - The current position of the piece as a bitboard.
    /// * `target_square` - The target position of the piece as a bitboard.
    /// * `piece` - The type of the piece being moved.
    /// * `color` - The color of the piece being moved.
    ///
    /// # Returns
    ///
    /// * `Ok(Board)` if the move is valid and the new board state.
    /// * `Err(String)` if the move leaves the king in check, with an error message describing the reason.
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
    ///
    /// This function updates the turn, halfmove clock, and fullmove number.
    fn finalize_turn(&mut self) {
        self.halfmove_clock += 1;
        self.white_turn = !self.white_turn;
    }

    /// Get the boards for the current player and the opponent
    ///
    /// # Returns
    ///
    /// * A tuple containing references to the current player's board and the opponent's board.
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

    /// Returns the possible legal moves for a piece at the given square.
    ///
    /// # Arguments
    /// * `current_square` - A `u64` representing the current square of the piece.
    ///
    /// # Returns
    /// * `Ok(u64)` - A `u64` bitboard representing the possible legal moves.
    /// * `Err(String)` - An error message if there is no piece at the current square.
    ///
    /// # Example
    /// ```
    /// let mut engine = Engine::new();
    /// let moves = engine.get_moves(coordinates_to_u64((6, 4))).unwrap();
    /// ```
    pub fn get_moves(&self, current_square: u64) -> Result<u64, String> {
        let (player_board, opponent_board) = self.get_half_turn_boards();
        let piece_type = get_piece_type(player_board, current_square);

        // Ensure there is a piece at the current square
        if piece_type.is_none() {
            return Err("No piece at this location".to_string());
        }

        let piece = piece_type.unwrap();
        let color = get_color(self.white_turn);

        // Get the possible moves for the piece
        let legal_moves = get_possible_move(
            &piece,
            current_square,
            player_board.bitboard(),
            opponent_board.bitboard(),
            &color,
        );

        // Initialize a bitboard for filtered moves
        let mut possible_moves = 0u64;
        let mut moves_to_check = legal_moves;

        // Iterate through each set bit in legal_moves
        while moves_to_check != 0 {
            // Get the least significant set bit
            let target_square = 1u64 << moves_to_check.trailing_zeros();

            // If the move doesn't leave king in check, add it to possible moves
            if self
                .simulate_and_check_move(current_square, target_square, &piece, &color)
                .is_ok()
            {
                possible_moves |= target_square;
            }

            // Clear the processed bit
            moves_to_check &= !target_square;
        }

        Ok(possible_moves)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Resets the engine to the initial state.
    pub fn reset(&mut self) {
        self.board = Board::new();
        self.white_turn = true;
        self.halfmove_clock = 0;
    }

    /// Returns the number of full moves in the game.
    ///
    /// # Returns
    /// A `u32` representing the number of full moves.
    pub fn fullmove_number(&self) -> u32 {
        (self.halfmove_clock + 1) / 2
    }

    /// Returns the number of halfmoves since the last pawn move or capture.
    ///
    /// # Returns
    /// A `u32` representing the number of halfmoves.
    pub fn halfmove_clock(&self) -> u32 {
        self.halfmove_clock
    }
}
