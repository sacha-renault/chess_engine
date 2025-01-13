use super::board::Board;
use super::color::Color;
use super::pieces::Piece;
use super::static_positions;

fn piece_to_char(piece: Piece, color: Color) -> char {
    let piece_char = match piece {
        Piece::King => 'K',
        Piece::Queen => 'Q',
        Piece::Rook => 'R',
        Piece::Bishop => 'B',
        Piece::Knight => 'N',
        Piece::Pawn => 'P',
    };
    // Capitalize for white pieces, lowercase for black pieces
    if color == Color::White {
        piece_char
    } else {
        piece_char.to_ascii_lowercase()
    }
}

pub fn print_board(board: &Board) {
    // Print column headers
    print!("    ");
    for col in 0..8 {
        print!(" {} ", col);
    }
    println!();

    for row in (0..8).rev() {
        // Reverse the row order for correct display (8 to 1)
        print!("{} | ", row); // Print row number
        for col in 0..8 {
            let square = 1 << (row * 8 + col);
            let mut piece_displayed = false;

            // Default background (alternating black and white)
            let bg_color = if (row + col) % 2 == 0 {
                static_positions::WHITE_BG
            } else {
                static_positions::BLACK_BG
            };

            // Check and print each piece for both white and black
            for piece in [
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ]
            .iter()
            {
                let white_bitboard = board.white.get_bitboard_by_type(piece.clone());
                let black_bitboard = board.black.get_bitboard_by_type(piece.clone());

                // Check if the piece exists in this square for white pieces
                if (white_bitboard & square) != 0 {
                    print!(
                        "{} {} {}",
                        bg_color,
                        piece_to_char(piece.clone(), Color::White),
                        static_positions::RESET
                    );
                    piece_displayed = true;
                    break;
                }
                // Check if the piece exists in this square for black pieces
                if (black_bitboard & square) != 0 {
                    print!(
                        "{} {} {}",
                        bg_color,
                        piece_to_char(piece.clone(), Color::Black),
                        static_positions::RESET
                    );
                    piece_displayed = true;
                    break;
                }
            }

            // If no piece is found, print an empty space with the background
            if !piece_displayed {
                print!("{}   {}", bg_color, static_positions::RESET);
            }
        }
        println!(); // Move to the next line after each row
    }
}

pub fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let bit = 1u64 << (rank * 8 + file);
            if bitboard & bit != 0 {
                print!(" X ");
            } else {
                print!(" . ");
            }
        }
        println!(); // Move to the next rank
    }
}
