use super::move_results::{CorrectMoveResults, IncorrectMoveResults, MoveResult};
use super::player_move::{CastlingMove, PlayerMove, PromotionMove};
use super::utility::{
    get_color, get_en_passant_ranks, get_final_castling_positions, get_half_turn_boards, get_half_turn_boards_mut, get_initial_castling_positions, get_piece_type, get_possible_move, get_promotion_rank_by_color, get_required_empty_squares, is_king_checked, iter_into_u64, move_piece, is_promotion_available
};
use crate::boards::Board;
use crate::game_engine::debug::{print_bitboard, print_board};
use crate::pieces::piece::PROMOTE_PIECE;
use crate::pieces::Color;
use crate::pieces::Piece;
use crate::prelude::NormalMove;
use super::get_move_row::GetMoveRow;

/// Represents a chess engine that manages game state and move validation.
///
/// The engine handles:
/// - Game state (board position, turn, move counters)
/// - Move validation and execution
/// - Special moves (castling, promotion)
/// - Move generation and validation
#[derive(Debug, Clone)]
pub struct Engine {
    // rules
    board: Board,
    white_turn: bool,
    halfmove_clock: u32, // Number of halfmoves since the last pawn move or capture
}

impl Engine {
    /// Creates a new chess engine with the standard starting position.
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
            halfmove_clock: 0,
        }
    }

    pub fn clone_with_new_board(&self, board: Board) -> Self {
        Engine {
            board: board,
            white_turn: self.white_turn,
            halfmove_clock: self.halfmove_clock,
        }
    }

    pub fn white_to_play(&self) -> bool {
        self.white_turn
    }

    /// Executes a chess move, handling both normal moves and castling.
    ///
    /// # Arguments
    /// * `chess_move` - The move to execute, either normal move or castling
    ///
    /// # Returns
    /// * `Ok(CorrectMoveResults)` - Move executed successfully
    /// * `Err(IncorrectMoveResults)` - Move validation failed
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * There is no piece at the current position.
    /// * The target position is not a valid move for the piece.
    /// * The move leaves the king in check.
    pub fn play(&mut self, chess_move: PlayerMove) -> MoveResult {
        // else we can play normal
        self.board = match chess_move {
            PlayerMove::Normal(normal_move) => {
                // get squares and color
                let (current_square, target_square) = normal_move.squares();
                let new_board = self.perform_move(current_square, target_square)?;

                // here we ensure the piece moved wasn't a pawn on promotion rank
                // if it was, we return an error
                if is_promotion_available(&new_board, target_square, get_color(self.white_turn)) {
                    return Err(IncorrectMoveResults::PromotionExpected);
                }
                new_board
            }
            PlayerMove::Castling(castling_side) => {
                // perform casting
                self.perform_castling(castling_side)?
            }
            PlayerMove::Promotion(promotion_move)    => {
                // get squares
                let (current_square, target_square) = promotion_move.squares();
                self.board = self.perform_move(current_square, target_square)?;
                self.promote_pawn(promotion_move.promotion_piece(), target_square)?
            }
        };

        // Finalize the turn
        Ok(self.finalize_turn())
    }

    /// Validates and simulates a move before execution.
    ///
    /// Checks if:
    /// - There is a piece at the starting square
    /// - The move is legal for the piece
    /// - The move doesn't leave the king in check
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

        // Get piece + color
        let piece = match piece_type {
            Some(p) => p,
            None => return Err(IncorrectMoveResults::NoPieceAtLocation),
        };
        let color = get_color(self.white_turn);

        // Get the possible moves for the piece
        let possible_moves = get_possible_move(
            piece,
            current_square,
            player_board.bitboard(),
            opponent_board.bitboard(),
            opponent_board.en_passant,
            color,
        );

        // Check if the target square is a valid move
        if target_square & possible_moves == 0 {
            return Err(IncorrectMoveResults::IllegalMove);
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
        piece: Piece,
        color: Color,
    ) -> Result<Board, IncorrectMoveResults> {
        // Simulate the move
        let mut simulated_board = move_piece(
            self.board.clone(),
            current_square,
            target_square,
            color,
            piece,
        );

        // perform en passant squares check
        self.handle_en_passant(&mut simulated_board, current_square, target_square);

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
    /// It also checks if there isn't running promotions
    fn finalize_turn(&mut self) -> CorrectMoveResults {
        // get the color
        let color = get_color(self.white_turn);

        // get player and opponent board
        let (player_board, opponent_board) = get_half_turn_boards_mut(&mut self.board, color);

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

        // Check if there is still any pawns on the last rank
        // a promotion move is needed instead of normal move
        // if player_board.pawn & get_promotion_rank_by_color(color) != 0 {
        //      return IncorrectMoveResults::PromotionExpected;
        // }

        // reset the en passant squares for the opponent
        opponent_board.en_passant = 0;

        // we get the initial position depending on the color
        self.halfmove_clock += 1;
        self.white_turn = !self.white_turn;

        CorrectMoveResults::Ok
    }

    /// Handles all en passant-related logic after a move, including both setting up and executing en passant captures.
    ///
    /// This function serves two purposes:
    /// 1. If a pawn makes a two-square advance, it sets up the en passant opportunity
    /// 2. If a pawn captures via en passant, it removes the captured pawn from the board
    ///
    /// # Arguments
    ///
    /// * `board` - Mutable reference to the game board
    /// * `current_square` - The starting square of the move (as a bitboard with single bit set)
    /// * `target_square` - The destination square of the move (as a bitboard with single bit set)
    ///
    /// # Note
    ///
    /// This function assumes the move has already been executed on the board,
    /// meaning the moving piece should already be at the target square when this
    /// function is called.
    fn handle_en_passant(&self, board: &mut Board, current_square: u64, target_square: u64) {
        // get the color and check if the current move can produce en passant square
        let color = get_color(self.white_turn);
        let ep_ranks = get_en_passant_ranks(color);
        let (player_board, opponent_board) = get_half_turn_boards_mut(board, color);

        // we first ensure the piece move is a pawn
        // piece is already moved so it's located at destination square
        if player_board.pawn & target_square == 0 {
            return;
        }

        // We check if ranks & moves is exactly 2 (start and end positions)
        // otherwise, we can't have any en passant
        if (ep_ranks & (current_square | target_square)).count_ones() == 2 {
            // If we got here, we have a valid two-square pawn move
            // Set the en passant square to the square the pawn passed over
            player_board.en_passant = match color {
                Color::White => target_square >> 8,
                Color::Black => target_square << 8,
            };
        }
        // in this case we took the pawn with en passant
        // if yes, we have to remove the pawn
        else if opponent_board.en_passant & target_square != 0 {
            // retrieve the position of the pawn that triggered en passant
            let pawn_position = match color {
                Color::White => target_square >> 8,
                Color::Black => target_square << 8,
            };

            // we delete the pawn from the board
            opponent_board.pawn &= !pawn_position
        }
    }

    /// Performs a castling move for the current player.
    ///
    /// # Arguments
    ///
    /// * `castling` - A `CastlingMove` enum indicating whether it's a long (queenside) or short (kingside) castling.
    ///
    /// # Returns
    ///
    /// * `Ok(Board)` - A new board state if the castling move is valid and successfully executed.
    /// * `Err(IncorrectMoveResults)` - An error if the castling is not allowed, with `CastlingNotAllowed` as the reason.
    ///
    /// # Details
    ///
    /// This function:
    /// 1. Verifies if castling is available based on:
    ///    - Required squares being empty
    ///    - Castling rights being maintained
    /// 2. Moves both the king and rook to their respective positions
    /// 3. Ensures the king is not in check after the move
    fn perform_castling(&self, castling: CastlingMove) -> Result<Board, IncorrectMoveResults> {
        // get color
        let color = get_color(self.white_turn);

        // get player and opponent board
        let (player_board, opponent_board) = get_half_turn_boards(&self.board, color);

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

        if can_castle && !is_king_checked(player_board.king, player_board, opponent_board, color) {
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
                Piece::King,
            );

            // simutate move of rook
            let simulated_board = move_piece(
                board_intermediate.clone(),
                initial_rook_pos,
                final_rook_pos,
                color,
                Piece::Rook,
            );

            // Get the simulated player's and opponent's boards
            let (sim_player_board, sim_opponent_board) =
                get_half_turn_boards(&simulated_board, get_color(!self.white_turn));

            // Check if the king is in check in the simulated state
            if !is_king_checked(
                sim_opponent_board.king,
                &sim_player_board,
                &sim_opponent_board,
                get_color(!self.white_turn),
            ) {
                return Ok(simulated_board);
            }
        }

        Err(IncorrectMoveResults::CastlingNotAllowed)
    }

    /// Returns all legal moves for a piece at the given square.
    ///
    /// # Arguments
    /// * `current_square` - Bitboard with a single bit set representing the piece's position
    ///
    /// # Returns
    /// * `Ok(u64)` - Bitboard where set bits represent legal destination squares
    /// * `Err(String)` - Error if no piece exists at the square
    pub fn get_moves(&self, current_square: u64) -> Result<u64, String> {
        let (player_board, opponent_board) =
            get_half_turn_boards(&self.board, get_color(self.white_turn));
        let piece_type = get_piece_type(player_board, current_square);

        // Ensure there is a piece at the current square
        if piece_type.is_none() {
            return Err("No piece at this location".to_string());
        }

        // get piece and color
        let piece = match piece_type {
            Some(p) => p,
            None => return Err("No piece at this location".to_string()),
        };
        let color = get_color(self.white_turn);

        // Get the possible moves for the piece
        let legal_moves = get_possible_move(
            piece,
            current_square,
            player_board.bitboard(),
            opponent_board.bitboard(),
            opponent_board.en_passant,
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

    /// Returns is king checked for the ones who has to move
    pub fn is_king_checked(&self) -> bool {
        let color = get_color(self.white_turn);
        let (player_board, opponent_board) = get_half_turn_boards(&self.board, color);
        is_king_checked(player_board.king, &opponent_board, &player_board, color)
    }

    /// Promotes a pawn that has reached the opposite end of the board.
    ///
    /// # Arguments
    /// * `piece` - The piece type to promote the pawn to
    ///
    /// # Returns
    /// * `Ok(CorrectMoveResults)` - Promotion successful
    /// * `Err(IncorrectMoveResults)` - Promotion not possible
    fn promote_pawn(&self, piece: Piece, target_square: u64) -> Result<Board, IncorrectMoveResults> {
        // get color
        let color = get_color(self.white_turn);

        // we change the piece at the location
        let mut simulated_board = self.board.clone();

        // Get the board
        let (player_board, _) = get_half_turn_boards_mut(&mut simulated_board, color);

        // we check if there should be a promotion
        if is_promotion_available(&self.board, target_square, color) {
            // we get the pawns on the player board and we remove it
            // then we add the new piece
            player_board.pawn &= !target_square; // remove the pawns from the square
            player_board.set_bitboard_by_type(
                piece,
                player_board.get_bitboard_by_type(piece) | target_square,
            );

            Ok(simulated_board)

        } else {
            return Err(IncorrectMoveResults::IllegalPromotion);
        }
    }

    /// Returns all possible moves for all pieces of the current player.
    ///
    /// This function calculates all legal moves for each piece belonging to the current player
    /// (determined by `white_turn`). For each piece, it returns its position, type, and a
    /// bitboard representing all its possible moves.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(Vec<(u64, Pieces, u64)>)`: A vector of tuples where each tuple contains:
    ///   - `u64`: The position of the piece as a bitboard (single bit set)
    ///   - `Piece`: The type of the piece (e.g., Pawn, Knight, etc.)
    ///   - `u64`: A bitboard representing all possible moves for this piece
    /// - `Err(String)`: An error message if move generation fails
    pub fn get_all_moves_by_piece(&self) -> Result<Vec<(Piece, PlayerMove)>, String> {
        // get the correct color board
        let color = get_color(self.white_turn);
        let (player_board, _) = get_half_turn_boards(&self.board, color);

        // then get all the pieces
        let pieces = player_board.individual_pieces();

        let pieces_with_moves = pieces
            .into_iter()
            .map(|it| self.get_moves(it.0).map(|moves| (it.1, PlayerMove::Normal(NormalMove::new(it.0, moves)))))
            .collect::<Result<Vec<_>, String>>()?;

        Ok(pieces_with_moves)
    }

    pub fn generate_moves_with_engine_state(&self) -> Result<Vec<GetMoveRow>, String> {
        // get the correct color board
        let color = get_color(self.white_turn);
        let (player_board, opponent_board) =
            get_half_turn_boards(&self.board, color);

        // then get all the pieces
        let pieces = player_board.individual_pieces();

        // init a vector for result
        let mut result = Vec::new();

        // iteratin INTO the pieces
        for (current_square, piece) in pieces.into_iter() {
            // Get the possible moves for the piece
            let pseudo_legal_moves = get_possible_move(
                piece,
                current_square,
                player_board.bitboard(),
                opponent_board.bitboard(),
                opponent_board.en_passant,
                color,
            );

            // get promotion rnak
            let promotion_rank = get_promotion_rank_by_color(color);

            // iterate over the legal moves
            for target_index in iter_into_u64(pseudo_legal_moves) {
                // Get the least significant set bit
                let target_square = 1u64 << target_index;

                // If the move doesn't leave king in check, add it to possible moves
                match self.validate_move_safety(current_square, target_square, piece, color) {
                    Ok(board) => {
                        // in the case the move is valid, we just as if we would for a normal move
                        let mut engine = self.clone_with_new_board(board);

                        // check if the move is a promotion
                        if piece == Piece::Pawn && target_square & promotion_rank != 0 {
                            for promotion_piece in PROMOTE_PIECE {
                                // clone the engine to perform each the promotion
                                let promotion_engine = engine.clone();
                                let new_board = promotion_engine.promote_pawn(promotion_piece, target_square).unwrap();
                                let mut final_engine = engine.clone_with_new_board(new_board);
                                let move_result = final_engine.finalize_turn();

                                // add the moverow to the vec
                                result.push(GetMoveRow {
                                    engine: final_engine,
                                    player_move: PlayerMove::Promotion(PromotionMove::new(current_square, target_square, promotion_piece)),
                                    piece,
                                    color,
                                    result: move_result,
                                })
                            }
                        } else {
                            // get the move result
                            let move_result = engine.finalize_turn();

                            // add the moverow to the vec
                            result.push(GetMoveRow {
                                engine,
                                player_move: PlayerMove::Normal(NormalMove::new(current_square, target_square)),
                                piece,
                                color,
                                result: move_result
                            })
                        }
                    }
                    _ => { /* Nothing to do ... just sad this move won't work right ? */}
                }
            }
        }

        Ok(result)
    }
}
