use super::board::{Board, ColorBoard};
use super::pieces::Pieces;
use super::static_positions;
use super::utility::{
    coordinates_to_u64, get_color, get_piece_type, get_possible_move, is_king_checked, move_piece,
};

pub struct Engine {
    // rules
    board: Board,
    white_turn: bool,
    en_passant: Option<usize>, // Optional field for en passant target square
    castling_rights: (bool, bool, bool, bool), // (white_king_side, white_queen_side, black_king_side, black_queen_side)
    halfmove_clock: u32, // Number of halfmoves since the last pawn move or capture
    fullmove_number: u32, // Number of full moves in the game
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::new(),
            white_turn: true,
            en_passant: None,
            castling_rights: (true, true, true, true),
            halfmove_clock: 0,
            fullmove_number: 0,
        }
    }

    pub fn play(&mut self, current: (usize, usize), target: (usize, usize)) -> Result<(), String> {
        // get user input as u64 position
        let current_square = coordinates_to_u64(current);
        let target_square = coordinates_to_u64(target);

        // get boards & bitboards for this turn
        let (board, opponent_board) = self.get_half_turn_boards();
        let same_color_bitboard = board.bitboard();
        let other_color_bitboard = opponent_board.bitboard();

        // get the piece type and early exit if nothing found
        let piece_type = get_piece_type(board, current_square);
        println!("{:?}", piece_type);

        // ensure a piece can be moved at this location
        if piece_type.is_none() {
            return Err("No piece at this location".to_string());
        }

        // get color for this turn
        let piece = piece_type.unwrap();
        let color = get_color(self.white_turn);

        // get possible move for this piece
        let possible_moves = get_possible_move(
            &piece,
            current_square,
            same_color_bitboard,
            other_color_bitboard,
            &color,
        );

        // if target square is in the possible moves, then we can play
        if target_square & possible_moves == 0 {
            return Err("This isn't a possible move".to_string());
        }

        // if this is okay, we copy the board and make the move
        let mut next_board = self.board.clone();

        // we compute the move
        next_board = move_piece(next_board, current_square, target_square, &color, &piece);

        // Compute all the moves and ensure king isn't checked
        if is_king_checked(board.king, opponent_board, board, &color) {
            return Err("King is checked".to_string());
        }

        // if we arrive here is means it's all good, now we just have
        // to replace the current board with next board and change turn
        self.board = next_board;
        self.halfmove_clock += 1;
        self.white_turn = !self.white_turn;

        // We have some more extra steps after i know
        Ok(())
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

    pub fn pawn_moves(&self) -> u64 {
        todo!()
    }

    pub fn rooks_moves(&self) -> u64 {
        todo!()
    }

    pub fn strait_moves(&self) -> u64 {
        todo!()
    }

    pub fn diagonal_moves(&self) -> u64 {
        todo!()
    }

    pub fn queen_moves(&self) -> u64 {
        self.strait_moves() | self.diagonal_moves()
    }

    pub fn king_moves(&self) -> u64 {
        todo!()
    }
}
