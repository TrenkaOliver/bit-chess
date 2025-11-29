pub const FILE_A:  u64 = 0x0101_0101_0101_0101;
pub const FILE_B:  u64 = 0x0202_0202_0202_0202;
pub const FILE_G:  u64 = 0x4040_4040_4040_4040;
pub const FILE_H:  u64 = 0x8080_8080_8080_8080;

pub const RANK_0: u64 = 0x0000_0000_0000_00FF;
pub const RANK_7: u64 = 0xFF00_0000_0000_0000;

pub const FILE_AB: u64 = FILE_A | FILE_B;
pub const FILE_GH: u64 = FILE_G | FILE_H;

pub const RANK_1_OR_7: u64 = RANK_0 | RANK_7; 

pub const NOT_FILE_A:  u64 = !FILE_A;
pub const NOT_FILE_AB: u64 = !FILE_AB;
pub const NOT_FILE_H:  u64 = !FILE_H;
pub const NOT_FILE_GH: u64 = !FILE_GH;

pub const NOT_RANK_0: u64 = !RANK_0;
pub const NOT_RANK_7: u64 = !RANK_7;

pub const O_O_PIECES: u64   = FILE_H | WHITE_KING | BLACK_KING;
pub const O_O_O_PIECES: u64 = FILE_A | WHITE_KING | BLACK_KING;
pub const WHITE_KINGSIDE_MASK:  u64 = (1u64 << 5)  | (1u64 << 6);
pub const WHITE_QUEENSIDE_MASK: u64 = (1u64 << 1)  | (1u64 << 2) | (1u64 << 3);
pub const BLACK_KINGSIDE_MASK:  u64 = (1u64 << 61) | (1u64 << 62);
pub const BLACK_QUEENSIDE_MASK: u64 = (1u64 << 57) | (1u64 << 58) | (1u64 << 59);

pub const WHITE_PAWNS: u64   = 0b1111_1111_u64 << 8;
pub const WHITE_KNIGHTS: u64 = 0b0100_0010_u64;
pub const WHITE_BISHOPS: u64 = 0b0010_0100_u64;
pub const WHITE_ROOKS: u64   = 0b1000_0001_u64;
pub const WHITE_QUEENS: u64  = 0b0000_1000_u64;
pub const WHITE_KING: u64    = 0b0001_0000_u64;

pub const BLACK_PAWNS: u64   = WHITE_PAWNS << 40;
pub const BLACK_KNIGHTS: u64 = WHITE_KNIGHTS << 56;
pub const BLACK_BISHOPS: u64 = WHITE_BISHOPS << 56;
pub const BLACK_ROOKS: u64   = WHITE_ROOKS << 56;
pub const BLACK_QUEENS: u64  = WHITE_QUEENS << 56;
pub const BLACK_KING: u64    = WHITE_KING << 56;

mod check_detection;
mod legal_moves;

pub use check_detection::*;
pub use legal_moves::*;

//processs input coordinates, returns a tuple
//0: rank, 1: file
#[inline]
pub fn rank_and_file(input: &str) -> (u8, u8) {
    let mut chars = input.chars();
    let file = chars.next().unwrap().to_ascii_uppercase() as u8;
    let rank = chars.next().unwrap() as u8;
    (
        rank - 49, //rank
        file - 65, //file
    )
}

//ORs the masks in a slice
#[inline]
pub fn get_unified_mask(pieces: &[u64]) -> u64 {
    let mut result: u64 = 0;
    for piece in pieces.iter() {
        result |= piece;
    }
    result
}

//prints the chessboard
#[inline]
pub fn print_table(board: &[u64]) {
    for rank in (0..8).rev() {
        let rank_value = rank * 8;
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = 1u64 << (rank_value + file);
            if board[0] & square != 0 {
                print!("♙ ");
            } else if board[6] & square != 0 {
                print!("♟ ");
            } else if board[1] & square != 0 {
                print!("♘ ");
            } else if board[7] & square != 0 {
                print!("♞ ");
            } else if board[2] & square != 0 {
                print!("♗ ");
            } else if board[8] & square != 0 {
                print!("♝ ");
            } else if board[3] & square != 0 {
                print!("♖ ");
            } else if board[9] & square != 0 {
                print!("♜ ");
            } else if board[4] & square != 0 {
                print!("♕ ");
            } else if board[10] & square != 0 {
                print!("♛ ");
            } else if board[5] & square != 0 {
                print!("♔ ");
            } else if board[11] & square != 0 {
                print!("♚ ");
            } else {
                print!("・");
            }
        }
        println!();
    }
    println!("  A B C D E F G H\n");
}

pub fn debug_mask(mask: u64) {
    for rank in (0..8).rev() {
        let rank_value = rank * 8;
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = 1u64 << (rank_value + file);
            if mask & square != 0 {
                print!("♙ ");
            } else {
                print!("・");
            }
        }
        println!();
    }
    println!("  A B C D E F G H\n");
}