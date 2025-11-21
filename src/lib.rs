mod check_detection;

pub use check_detection::is_in_check;

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


