mod check_detection;

use check_detection::*;

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


///decides whether king is in check
#[inline]
pub fn is_in_check(king: u64, board: u64, pawns: u64, knights: u64, diagnal: u64, linear: u64, is_white: bool) -> bool {
    //get kings pos:
    let bit_idx = king.trailing_zeros() as i8;
    let rank = bit_idx / 8;
    let file = bit_idx % 8;


    //KNIGHT
    //create knight mask:
    let mut knight_mask = 0u64;

    //if knight's range isn't off-board add to mask
    if rank > 0 && file > 1 { knight_mask |= king >> 10; }
    if rank > 0 && file < 6 { knight_mask |= king >> 6;  }
    if rank > 1 && file > 0 { knight_mask |= king >> 17; }
    if rank > 1 && file < 7 { knight_mask |= king >> 15; }
    if rank < 6 && file > 0 { knight_mask |= king << 15; }
    if rank < 6 && file < 7 { knight_mask |= king << 17; }
    if rank < 7 && file > 1 { knight_mask |= king << 6;  }
    if rank < 7 && file < 6 { knight_mask |= king << 10; }

    //check against knight mask:
    if knight_mask & knights != 0 {return true;}


    //DIAGNAL
    //create a mask which represent every piece that cannot take diagnal;
    let other = board & (diagnal ^ u64::MAX);

    //create diagnal (bishop + queen's diagnal threat) mask:
    let mut diagnal_mask = 0u64;

    //range ends (exclusive) (ranges inside fn starts from 0 to exclude the king's position)
    //means how much squares do we need to check for threatening pieces
    //with mp (multiplier) we can specify patterns

    //from king to top right
    let range_end = 8 - rank.max(file);
    check_direction_pos(range_end, &mut diagnal_mask, 9, king, other);

    //from king to top left
    let range_end = if file < 7 - rank {file} else {rank};
    check_direction_pos(range_end, &mut diagnal_mask, 7, king, other);

    //from king to bottom right
    let range_end = if rank < 7 - file {rank} else {file};
    check_direction_neg(range_end,&mut diagnal_mask, 7, king, other);

    //from king to bottm left
    let range_end = rank.min(file);
    check_direction_neg(range_end, &mut diagnal_mask, 9, king, other);

    //check against mask:
    if diagnal_mask & diagnal != 0 {return true;}


    //LINEAR
    //recreate other (similar just no it doesn't contain linear hitting pieces)
    let other = board & (linear ^ u64::MAX);

    //create linear mask:
    let mut linear_mask = 0u64;

    //from king to top
    check_direction_pos(7 - rank, &mut linear_mask, 8, king, other);

    //from king to right
    check_direction_pos(7 - file, &mut linear_mask, 1, king, other);

    //from king to down 
    check_direction_neg(rank + 1, &mut linear_mask, 8, king, other);

    //from king to left
    check_direction_neg(file + 1, &mut linear_mask, 1, king, other);

    //check against mask:
    if linear_mask & linear != 0 {return true;}

    
    //PAWN
    //create pawn mask:
    let pawn_mask = if is_white {
        (king << 7) | (king << 9)
    } else {
        (king >> 9) | (king >> 7)
    };

    //check against mask
    if pawn_mask & pawns != 0 {return true;}
    

    //if there was no check return false
    false
}