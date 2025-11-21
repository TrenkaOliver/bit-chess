//decides whether king is in check
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


//adds to mask in pos direction (shifting kind left)
//mult is needed for patterns (diagnal, horizontal, vertical)
//king is the kings position, other is everything else than the type (diagnal/linear) the mask is built for
#[inline]
pub fn check_direction_pos(range_end: i8, mask: &mut u64, mp: i8, king: u64, other: u64) {
    for i in 1..range_end {
        if shift_left(mask, i * mp, king, other) {
            break;
        }
    }
}

//same as previous just checks in neg direction (shifts king right)
#[inline]
pub fn check_direction_neg(range_end: i8, mask: &mut u64, mp: i8, king: u64, other: u64) {
    for i in 1..range_end {
        if shift_right(mask, i * mp, king, other) {
            break;
        }
    }
}

//mask is the mask thats being built for a type of pieces (diagnal/linear)
//shifts the king's position in the given amount to the left and checks if there's a neutral piece in the way
//if there is, returns true, meaning we should break out from the loops above
//if there isn't than merges the current mask with the kings modified position (moved)
#[inline]
pub fn shift_left(mask: &mut u64, shift: i8, king: u64, other: u64) -> bool {
    let moved = king << shift;
    if moved & other == 0 {
        *mask |= moved;
        return false;
    }

    true
}

//similar as shift_left, the difference is this shifts the king's pos right
#[inline]
pub fn shift_right(mask: &mut u64, shift: i8, king: u64, other: u64) -> bool {
    let moved = king >> shift;
    if moved & other == 0 {
        *mask |= moved;
        return false;
    }

    true
}