use super::player_move::{CastlingMove, NormalMove, PlayerMove, PromotionMove};
use crate::{boards::Board, pieces::{Color, Piece}};

pub fn parse_str_into_square(target_file: char, target_rank: char) -> Result<u64, ()> {
    // Validate the input
    if !('a'..='h').contains(&target_file) || !('1'..='8').contains(&target_rank) {
        Err(())
    } else {
        let target =
            1u64 << ((target_rank as u64 - '1' as u64) * 8 + (target_file as u64 - 'a' as u64));
        Ok(target)
    }
}

pub fn parse_opt_source_file_and_rank(
    piece: Piece,
    chars: Vec<char>,
) -> (Option<char>, Option<char>) {
    match (piece, chars.len()) {
        (Piece::Pawn, 3) => (Some(chars[0]), None),
        (_, 4) => {
            if ('a'..='h').contains(&chars[1]) {
                (Some(chars[1]), None)
            } else {
                (None, Some(chars[1]))
            }
        }
        (_, 5) => (Some(chars[1]), Some(chars[2])),
        _ => (None, None),
    }
}

pub fn square_to_file(pos: u64) -> char {
    let square_index = pos.trailing_zeros() as u8;
    let file = (square_index % 8) as u8 + b'a';
    file as char
}

pub fn square_to_rank(pos: u64) -> char {
    let square_index = pos.trailing_zeros() as u8;
    let rank = (square_index / 8) as u8 + b'1';
    rank as char
}

pub fn match_piece_by_char(c: char) -> Piece {
    match c {
        'K' => Piece::King,
        'Q' => Piece::Queen,
        'R' => Piece::Rook,
        'B' => Piece::Bishop,
        'N' => Piece::Knight,
        _ => Piece::Pawn,
    }
}

pub fn matches_move(
    mv_piece: Piece,
    mv_current_square: u64,
    mv_target_squares: u64,
    piece: Piece,
    target_square: u64,
    from_file: Option<char>,
    from_rank: Option<char>,
) -> bool {
    if mv_piece != piece {
        return false;
    }
    // Match target square
    if target_square & mv_target_squares == 0 {
        return false;
    }

    // File disambiguation
    if let Some(file) = from_file {
        if square_to_file(mv_current_square) != file {
            return false;
        }
    }

    // Rank disambiguation
    if let Some(rank) = from_rank {
        if square_to_rank(mv_current_square) != rank {
            return false;
        }
    }

    true
}

pub fn parse_castling(input: &str) -> Option<PlayerMove> {
    match input.to_uppercase().as_str() {
        "O-O" => Some(PlayerMove::Castling(CastlingMove::Short)),
        "O-O-O" => Some(PlayerMove::Castling(CastlingMove::Long)),
        _ => None,
    }
}

pub fn parse_input_string(input: &str) -> Result<(Vec<char>, Option<Piece>), ()> {
    let input = input.replace(&['+', '#', 'x'], "");
    let mut chars: Vec<char> = input.chars().collect();
    let promotion_opt = chars.iter().position(|c| *c == '=');
    let mut promotion_piece_opt = None;

    if let Some(eq_index) = promotion_opt {
        if eq_index != input.len() - 2 {
            return Err(());
        } else {
            let pc = chars.pop().unwrap();
            let promotion_piece = match_piece_by_char(pc);
            if promotion_piece == Piece::Pawn {
                return Err(());
            }
            promotion_piece_opt = Some(promotion_piece);

            chars.pop(); // Remove the '=' sign
        }
    }

    Ok((chars, promotion_piece_opt))
}

pub fn filter_possible_moves(
    possible_moves: Vec<(Piece, PlayerMove)>,
    piece: Piece,
    target_square: u64,
    from_file: Option<char>,
    from_rank: Option<char>,
) -> Vec<(Piece, PlayerMove)> {
    possible_moves
        .into_iter()
        .filter(|(pc, pm)| {
            // We first don't care about any other moves than noraml
            match pm {
                PlayerMove::Castling(_) => false,
                PlayerMove::Normal(mv) => {
                    let (mv_current_square, mv_target_squares) = mv.squares();
                    matches_move(
                        *pc,
                        mv_current_square,
                        mv_target_squares,
                        piece,
                        target_square,
                        from_file,
                        from_rank,
                    )
                }
                PlayerMove::Promotion(mv) => {
                    let (mv_current_square, mv_target_squares) = mv.squares();
                    matches_move(
                        *pc,
                        mv_current_square,
                        mv_target_squares,
                        piece,
                        target_square,
                        from_file,
                        from_rank,
                    )
                }
            }
        })
        .collect()
}

pub fn create_final_move(
    player_move: PlayerMove,
    promotion_piece_opt: Option<Piece>,
    target_square: u64,
) -> Result<PlayerMove, ()> {
    match (player_move, promotion_piece_opt) {
        (PlayerMove::Promotion(mv), Some(promotion_piece)) => Ok(PlayerMove::Promotion(
            PromotionMove::new(mv.squares().0, target_square, promotion_piece),
        )),
        (PlayerMove::Normal(mv), _) => Ok(PlayerMove::Normal(NormalMove::new(
            mv.squares().0,
            target_square,
        ))),
        _ => Err(()),
    }
}

// Helper function to convert a piece to its FEN character
pub fn piece_to_char(color: Color, piece: Piece) -> char {
    let piece_char = match piece {
        Piece::Pawn => 'P',
        Piece::Knight => 'N',
        Piece::Bishop => 'B',
        Piece::Rook => 'R',
        Piece::Queen => 'Q',
        Piece::King => 'K'
    };
    match color {
        Color::Black => piece_char.to_ascii_lowercase(),
        Color::White => piece_char
    }
}

pub fn fen_board_position(board: &Board) -> String {
    // init an empty string for the fen
    let mut board_position = String::new();

    // Process each rank from 8 to 1 (top to bottom)
    for rank in (0..8).rev() {
        let mut empty_squares = 0;

        // Process each file from a to h (left to right)
        for file in 0..8 {
            let square_index = rank * 8 + file;
            let square = 1u64 << square_index;

            match board.get_piece_at(square) {
                Some((color, piece)) => {
                    // If we had empty squares before this piece, add the count
                    if empty_squares > 0 {
                        board_position.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    board_position.push(piece_to_char(color, piece));
                }
                None => {
                    empty_squares += 1;
                }
            }
        }

        // Add any remaining empty squares at the end of the rank
        if empty_squares > 0 {
            board_position.push_str(&empty_squares.to_string());
        }

        // Add rank separator (/) except for the last rank
        if rank > 0 {
            board_position.push('/');
        }
    }

    board_position
}

pub fn fen_castling(board: &Board) -> String {
    let mut castling_rights = String::new();

    let mut has_castling = false;
    if board.white.castling_rights.is_short_castling_available() { castling_rights.push('K'); has_castling = true; }
    if board.white.castling_rights.is_long_castling_available() { castling_rights.push('Q'); has_castling = true; }
    if board.black.castling_rights.is_short_castling_available() { castling_rights.push('k'); has_castling = true; }
    if board.black.castling_rights.is_long_castling_available() { castling_rights.push('q'); has_castling = true; }
    if !has_castling { castling_rights.push('-'); }

    castling_rights
}

pub fn fen_en_passant(board: &Board) -> String {
    let mut en_passant = String::new();

    let en_passant_squares = board.white.en_passant | board.black.en_passant;
    if en_passant_squares != 0 {
        en_passant.push(square_to_file(en_passant_squares));
        en_passant.push(square_to_rank(en_passant_squares));
    } else {
        en_passant.push('-');
    }

    en_passant
}
