use super::board::Board;
use super::color::Color;
use super::move_results::{CorrectMoveResults, IncorrectMoveResults, MoveResult};
use super::pieces::Pieces;
use super::player_move::{CastlingMove, PlayerMove};
use super::utility::{
    get_color, get_final_castling_positions, get_half_turn_boards, get_half_turn_boards_mut,
    get_initial_castling_positions, get_piece_type, get_possible_move, get_promotion_rank_by_color,
    get_required_empty_squares, is_king_checked, move_piece,
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
    halfmove_clock: u32, // Number of halfmoves since the last pawn move or capture
    promoted: bool,
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
            halfmove_clock: 0,
            promoted: false,
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
    pub fn play(&mut self, chess_move: PlayerMove) -> MoveResult {
        // First check if there is any promotion
        if self.promoted {
            return Err(IncorrectMoveResults::WaitingForPromotion);
        }

        // else we can play normal
        match chess_move {
            PlayerMove::Normal(normal_move) => {
                // get squares
                let (current_square, target_square) = normal_move.squares();

                // Validate the move and get the new board state
                let new_board = self.perform_move(current_square, target_square)?;

                // Apply the new board state
                self.board = new_board;
            }
            PlayerMove::Castling(castling_side) => {
                // perform casting
                let new_board = self.perform_castling(castling_side)?;

                // Apply the new board state
                self.board = new_board;
            }
        };
        // Finalize the turn
        Ok(self.finalize_turn())
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
    fn perform_move(
        &self,
        current_square: u64,
        target_square: u64,
    ) -> Result<Board, IncorrectMoveResults> {
        // get player and opponent board
        let (player_board, opponent_board) =
            get_half_turn_boards(&self.board, get_color(self.white_turn));

        // Get piece type
        let piece_type = get_piece_type(player_board, current_square);

        // Ensure there is a piece at the current square
        if piece_type.is_none() {
            return Err(IncorrectMoveResults::NoPieceAtLocation);
        }

        let piece = piece_type.unwrap();
        let color = get_color(self.white_turn);

        // Get the possible moves for the piece
        let possible_moves = get_possible_move(
            piece,
            current_square,
            player_board.bitboard(),
            opponent_board.bitboard(),
            color,
        );

        // Check if the target square is a valid move
        if target_square & possible_moves == 0 {
            return Err(IncorrectMoveResults::IncorrectMove);
        }

        // Simulate the move and check if the king is in check
        let simulated_board =
            self.validate_move_safety(current_square, target_square, piece, color)?;

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
    fn validate_move_safety(
        &self,
        current_square: u64,
        target_square: u64,
        piece: Pieces,
        color: Color,
    ) -> Result<Board, IncorrectMoveResults> {
        // Simulate the move
        let simulated_board = move_piece(
            self.board.clone(),
            current_square,
            target_square,
            color,
            piece,
        );

        // Get the simulated player's and opponent's boards
        let (player_board, opponent_board) =
            get_half_turn_boards(&simulated_board, get_color(!self.white_turn));

        // Check if the king is in check in the simulated state
        // For that, we check all possible moves for next round (bulk computed all opponent moves)
        // and check if kinng_bitbord & all_moves == 0 => no check
        if is_king_checked(
            opponent_board.king,
            &player_board,
            &opponent_board,
            get_color(!self.white_turn),
        ) {
            return Err(IncorrectMoveResults::KingStillChecked);
        }
        Ok(simulated_board)
    }

    /// Finalize the turn after a move
    ///
    /// This function updates the turn, halfmove clock, and fullmove number adn castling rights.
    fn finalize_turn(&mut self) -> CorrectMoveResults {
        // get the color
        let color = get_color(self.white_turn);
        // get player and opponent board
        let (player_board, _) = get_half_turn_boards_mut(&mut self.board, color);

        // Get the initial position by color
        let (initial_king_pos, initial_short_rook_pos, initial_long_rook_pos) =
            get_initial_castling_positions(get_color(self.white_turn));

        // Update castling rights directly on the player's board
        player_board.castling_rights.update_castling_rights(
            player_board.king,
            player_board.rook,
            initial_king_pos,
            initial_short_rook_pos,
            initial_long_rook_pos,
        );

        // Check if the pawn is having promotion
        // in that case, we will raise a flag,
        // promotion needed
        if player_board.pawn & get_promotion_rank_by_color(color) != 0 {
            self.promoted = true;
            return CorrectMoveResults::Promote;
        }

        // we get the initial position depending on the color
        self.halfmove_clock += 1;
        self.white_turn = !self.white_turn;

        CorrectMoveResults::Ok
    }

    fn perform_castling(&mut self, castling: CastlingMove) -> Result<Board, IncorrectMoveResults> {
        // get color
        let color = get_color(self.white_turn);

        // get player and opponent board
        let (player_board, _) = get_half_turn_boards(&self.board, color);

        // get the full bitboard to ensure castling is available
        let full_bitboard = self.board.bitboard();

        // get castling empty required squares
        let required_empty: u64 = get_required_empty_squares(castling, color);

        // get initial positions
        let (initial_king_pos, initial_short_rook_pos, initial_long_rook_pos) =
            get_initial_castling_positions(color);

        // Check if can caslte
        let can_castle = match castling {
            CastlingMove::Long => {
                // Check if caslting is available
                player_board
                    .castling_rights
                    .long_casting_available(full_bitboard, required_empty)
            }

            CastlingMove::Short => {
                // Check if caslting is available
                player_board
                    .castling_rights
                    .short_casting_available(full_bitboard, required_empty)
            }
        };

        if can_castle {
            // get final positions
            let (final_king_pos, final_rook_pos) = get_final_castling_positions(castling, color);

            // match the initial rook pos
            let initial_rook_pos = match castling {
                CastlingMove::Long => initial_long_rook_pos,
                CastlingMove::Short => initial_short_rook_pos,
            };

            // Simulate the move of king
            let board_intermediate = move_piece(
                self.board.clone(),
                initial_king_pos,
                final_king_pos,
                color,
                Pieces::King,
            );

            // simutate move of rook
            let simulated_board = move_piece(
                board_intermediate.clone(),
                initial_rook_pos,
                final_rook_pos,
                color,
                Pieces::King,
            );

            // Get the simulated player's and opponent's boards
            let (player_board, opponent_board) =
                get_half_turn_boards(&simulated_board, get_color(!self.white_turn));

            // Check if the king is in check in the simulated state
            if !is_king_checked(
                opponent_board.king,
                &player_board,
                &opponent_board,
                get_color(!self.white_turn),
            ) {
                return Ok(simulated_board);
            }
        }

        Err(IncorrectMoveResults::CastlingNotAllowed)
    }

    /// Returns the possible legal moves for a piece at the given square.
    ///
    /// # Arguments
    /// * `current_square` - A `u64` representing the current square of the piece.
    ///
    /// # Returns
    /// * `Ok(u64)` - A `u64` bitboard representing the possible legal moves.
    /// * `Err(String)` - An error message if there is no piece at the current square.
    pub fn get_moves(&self, current_square: u64) -> Result<u64, String> {
        let (player_board, opponent_board) =
            get_half_turn_boards(&self.board, get_color(self.white_turn));
        let piece_type = get_piece_type(player_board, current_square);

        // Ensure there is a piece at the current square
        if piece_type.is_none() {
            return Err("No piece at this location".to_string());
        }

        let piece = piece_type.unwrap();
        let color = get_color(self.white_turn);

        // Get the possible moves for the piece
        let legal_moves = get_possible_move(
            piece,
            current_square,
            player_board.bitboard(),
            opponent_board.bitboard(),
            color,
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
                .validate_move_safety(current_square, target_square, piece, color)
                .is_ok()
            {
                possible_moves |= target_square;
            }

            // Clear the processed bit
            moves_to_check &= !target_square;
        }

        Ok(possible_moves)
    }

    // Utility methods
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn reset(&mut self) {
        *self = Self::new();
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
