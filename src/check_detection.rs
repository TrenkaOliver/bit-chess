
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