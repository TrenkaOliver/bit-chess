pub const FILE_A:  u64 = 0x0101_0101_0101_0101;
pub const FILE_B:  u64 = 0x0202_0202_0202_0202;
pub const FILE_G:  u64 = 0x4040_4040_4040_4040;
pub const FILE_H:  u64 = 0x8080_8080_8080_8080;

pub const RANK_0: u64 = 0x0000_0000_0000_00FF;
pub const RANK_7: u64 = 0xFF00_0000_0000_0000;

pub const FILE_AB: u64 = FILE_A | FILE_B;
pub const FILE_GH: u64 = FILE_G | FILE_H;

pub const RANK_17: u64 = RANK_0 | RANK_7; 

pub const NOT_FILE_A:  u64 = !FILE_A;
pub const NOT_FILE_AB: u64 = !FILE_AB;
pub const NOT_FILE_H:  u64 = !FILE_H;
pub const NOT_FILE_GH: u64 = !FILE_GH;

mod check_detection;
mod legal_moves;

pub use check_detection::*;
pub use legal_moves::*;

//processs input coordinates, returns a tuple
//0: rank, 1: file
#[inline]
pub fn rank_and_file(input: &str) -> (i8, i8) {
    let mut chars = input.chars();
    let file = chars.next().unwrap().to_ascii_uppercase() as i8;
    let rank = chars.next().unwrap() as i8;
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

pub fn print_mask(mask: u64) {
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