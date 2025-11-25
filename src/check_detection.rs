use crate::*;

//decides whether king is in check
#[inline]
pub fn is_in_check(king: u64, board: u64, pawns: u64, knights: u64, bishop_like: u64, rook_like: u64, is_white: bool) -> bool {
    //KNIGHT
    //check against knight mask:
    if get_knight_moves(king, 0) & knights != 0 {return true;}

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
    let bishop_mask = get_bishop_moves(king, other, 0)[0];

    //check against mask:
    if bishop_mask & bishop_like != 0 {return true;}


    //ROOK-LIKE
    //recreate other (similar just no it doesn't contain linear hitting pieces)
    let other = board & !rook_like;

    //create linear mask:
    let rook_mask = get_rook_moves(king, other, 0)[0];
    println!("exposed to rook-like moving pieces: ");
    print_mask(rook_mask);

    //check against mask:
    if rook_mask & rook_like != 0 {return true;}    

    //if there was no check return false
    false
}