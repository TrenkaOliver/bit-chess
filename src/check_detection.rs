use crate::*;



//decides whether king is in check
#[inline]
pub fn is_checked(king: u64, board: u64, pawns: u64, knights: u64, bishop_like: u64, rook_like: u64, is_white: bool) -> bool {
    //KNIGHT
    //check against knight mask:
    if get_knight_moves(king, 0)[0].1 & knights != 0 {return true;}

    //PAWN
    //create pawn mask:
    let pawn_mask = if is_white {
        (king << 7) | (king << 9)
    } else {
        (king >> 9) | (king >> 7)
    };

    //check against mask
    if pawn_mask & pawns != 0 {return true;}

    //BISHOP-LIKE
    //create a mask which represent every piece that cannot take diagnal;
    let other = board & !bishop_like;

    //create diagnal (bishop + queen's diagnal threat) mask:
    let bishop_mask = get_bishop_moves(king, other, 0)[0].1;

    //check against mask:
    if bishop_mask & bishop_like != 0 {return true;}

    //ROOK-LIKE
    //recreate other (similar, this doesn't contain not linear hitting pieces)
    let other = board & !rook_like;

    //create linear mask:
    let rook_mask = get_rook_moves(king, other, 0)[0].1;
    
    //check against mask:
    if rook_mask & rook_like != 0 {return true;}    

    //if there was no check return false
    false

}

pub fn check_slice(king_slice: &[u64], board: u64, pawns: u64, knights: u64, bishop_like: u64, rook_like: u64, is_white: bool) -> bool {
    for king in king_slice.iter() {
        if is_checked(*king, board, pawns, knights, bishop_like, rook_like, is_white) {
            return true;
        }
    }

    false
}


//validates moves so none of the moves leaves the king in check
pub fn validate_moves(legal_moves: &mut Vec<(usize, u64, u64)>, piece_type: usize, moves: &[(u64, u64)], board: &[u64; 12], idx: usize, opp_idx: usize, opp_mask: u64, is_white: bool) {
    for &(moved_piece, mut move_mask) in moves {
        let mut new_pos_idx = move_mask.trailing_zeros();
        while new_pos_idx != 64 {
            //create a temp board for check detection
            let new_pos = 1u64 << new_pos_idx;
            let mut temp_board = *board;
            temp_board[idx + piece_type] &= !moved_piece;
            temp_board[idx + piece_type] |= new_pos;

            //check if something is taken with this move
            if new_pos & opp_mask != 0 {
                for mut i in 0..6 {
                    i += opp_idx;
                    if new_pos & board[i] != 0 {
                        temp_board[i] &= !new_pos;
                        break;
                    }
                }
            }

            //check the temp board
            if !is_checked(
                temp_board[idx + 5], 
                get_unified_mask(&temp_board), 
                temp_board[opp_idx], 
                temp_board[opp_idx + 1], 
                temp_board[opp_idx + 2] | temp_board[opp_idx + 4], 
                temp_board[opp_idx + 3] | temp_board[opp_idx + 4], 
                is_white
            ) {
                legal_moves.push((piece_type, moved_piece, new_pos));
            }

            //remove the this move from movemask
            move_mask &= !new_pos;
            new_pos_idx = move_mask.trailing_zeros();
        }
    }
}