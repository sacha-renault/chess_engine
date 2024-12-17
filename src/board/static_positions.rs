const fn precompute_knight_moves() -> [u64; 64] {
    let mut moves = [0u64; 64];
    let mut square = 0;

    while square < 64 {
        let position = 1u64 << square;

        let mut knight_moves = 0u64;
        knight_moves |= (position << 17) & !FILE_A;  // Up 2, Right 1
        knight_moves |= (position << 15) & !FILE_H;  // Up 2, Left 1
        knight_moves |= (position << 10) & !(FILE_A | FILE_B); // Up 1, Right 2
        knight_moves |= (position << 6)  & !(FILE_G | FILE_H); // Up 1, Left 2

        knight_moves |= (position >> 17) & !FILE_H;  // Down 2, Left 1
        knight_moves |= (position >> 15) & !FILE_A;  // Down 2, Right 1
        knight_moves |= (position >> 10) & !(FILE_G | FILE_H); // Down 1, Left 2
        knight_moves |= (position >> 6)  & !(FILE_A | FILE_B); // Down 1, Right 2

        moves[square] = knight_moves;

        square += 1;
    }

    moves
}

const fn precompute_king_moves() -> [u64; 64] {
    let mut moves = [0u64; 64];
    let mut square = 0;

    while square < 64 {
        let position = 1u64 << square;

        let mut king_moves = 0u64;
        king_moves |= (position << 8); // North
        king_moves |= (position >> 8); // South

        king_moves |= (position << 1) & !FILE_A; // East
        king_moves |= (position >> 1) & !FILE_H; // West

        king_moves |= (position << 9) & !FILE_A; // North-East
        king_moves |= (position << 7) & !FILE_H; // North-West
        king_moves |= (position >> 7) & !FILE_A; // South-East
        king_moves |= (position >> 9) & !FILE_H; // South-West

        moves[square] = king_moves;
        square += 1;
    }

    moves
}

pub const WHITE_PAWNS: u64 = 0x000000000000FF00;
pub const BLACK_PAWNS: u64 = 0x00FF000000000000;

pub const WHITE_ROOKS: u64 = 0x0000000000000001 | 0x0000000000000080;
pub const WHITE_KNIGHTS: u64 = 0x0000000000000002 | 0x0000000000000040;
pub const WHITE_BISHOPS: u64 = 0x0000000000000004 | 0x0000000000000020;
pub const WHITE_QUEEN: u64 = 0x0000000000000008;
pub const WHITE_KING: u64 = 0x0000000000000010;

pub const BLACK_ROOKS: u64 = 0x0000000000000001 << 56 | 0x0000000000000080 << 56;
pub const BLACK_KNIGHTS: u64 = 0x0000000000000002 << 56 | 0x0000000000000040 << 56;
pub const BLACK_BISHOPS: u64 = 0x0000000000000004 << 56 | 0x0000000000000020 << 56;
pub const BLACK_QUEEN: u64 = 0x0000000000000008 << 56;
pub const BLACK_KING: u64 = 0x0000000000000010 << 56;

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = 0x0101010101010101;
pub const FILE_G: u64 = 0x8080808080808080;
pub const FILE_H: u64 = 0x8080808080808080;
pub const RANK3: u64 = 0x000000FF00000000;
pub const RANK6: u64 = 0x00000000FF000000;

pub const KNIGHTS_MOVES: [u64; 64] = precompute_knight_moves();
pub const KING_MOVES: [u64; 64] = precompute_king_moves();