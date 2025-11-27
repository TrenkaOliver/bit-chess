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

//returns attacking opponent's pos, than the squares between the attacing piece and king (exclusive) if the piece is sliding and a usize which represents the type
pub fn get_check_mask(king: u64, board: u64, pawns: u64, knights: u64, bishop_like: u64, rook_like: u64, queen: u64, is_white: bool, is_blocking: bool) -> Option<(u64, u64, usize)> {
    //KNIGHT
    //check against knight mask:
    let attacking_knight = get_knight_moves(king, 0)[0].1 & knights;
    if attacking_knight != 0 {return Some((attacking_knight, 0, 1));}

    //PAWN
    //create pawn mask:
    let pawn_mask = if !is_blocking {
        if is_white {
            (king << 7) | (king << 9)
        } else {
            (king >> 9) | (king >> 7)
        }
    } else {
        let start_pos = if is_white {WHITE_PAWNS} else {BLACK_PAWNS};
        if is_white {
            king << 8 & !board |
            ((king & start_pos) << 16) & !(board | board << 8)
        } else {
            king >> 8 & !board |
            ((king & start_pos) >> 16) & !(board | board >> 8)
        }
    };

    //check against mask
    let attacking_pawn = pawn_mask & pawns;
    if attacking_pawn != 0 {return Some((attacking_pawn, 0, 0));}

    //BISHOP
    //create a mask which represent every piece that cannot take diagnal;
    let other = board & !bishop_like;

    //create diagnal (bishop + queen's diagnal threat) mask:
    if let Some((a, p)) = get_first_checking_bishop_like(king, bishop_like, other, 0) {
        let t = if a & queen != 0 {4} else {2};
        return Some((a, p, t));
    }

    //check against mask:

    //ROOK
    //recreate other (similar, this doesn't contain not linear hitting pieces)
    let other = board & !rook_like;

    if let Some((a, p)) = get_first_checking_rook_like(king, rook_like, other, 0) {
        let t = if a & queen != 0 {4} else {3};
        return Some((a, p, t));
    }

    None    

}