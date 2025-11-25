use crate::*;

pub fn get_pawn_moves(pawns: u64, opp_mask: u64, uni_mask: u64, start_pos: u64, is_white: bool) -> u64 {
    //one forward; 2 forward; take in any dir
    if is_white {
        pawns << 8 & !uni_mask |
        ((pawns & start_pos) << 16) & !(uni_mask | uni_mask << 8) |
        pawns & (opp_mask >> 7 | opp_mask >> 9)
    } else {
        pawns >> 8 & !uni_mask |
        ((pawns & start_pos) >> 16) & !(uni_mask | uni_mask >> 8) |
        pawns & (opp_mask << 7 | opp_mask << 9)
    }
}

pub fn get_knight_moves(knights: u64, own_mask: u64) -> u64 {
    let knight_not_a = NOT_FILE_A & knights;
    let knight_not_ab = NOT_FILE_AB & knights;
    let knight_not_gh = NOT_FILE_GH & knights;
    let knight_not_h = NOT_FILE_H & knights;

    knight_not_a << 15 |
    knight_not_a >> 17 |
    knight_not_ab << 6 |
    knight_not_ab >> 10 |
    knight_not_gh << 10 |
    knight_not_gh >> 6 |
    knight_not_h << 17 |
    knight_not_h >> 15 &
    !own_mask

}

pub fn get_rook_moves(rooks: u64, own_mask: u64, opp_mask: u64) -> Vec<u64> {
    let mut out = vec![];
    let mut pieces = rooks;
    let exit_vertical = RANK_17 | opp_mask | own_mask;
    let exit_horizontal = FILE_A | FILE_H | opp_mask | own_mask;
    
    let mut idx = pieces.trailing_zeros();
    while idx != 64 {
        let mut moves = 0u64;
        let piece = 1u64 << idx;

        //up
        ls_piece(&mut moves, piece, 8, exit_vertical);
        //down
        rs_piece(&mut moves, piece, 8, exit_vertical);
        //right
        ls_piece(&mut moves, piece, 1, exit_horizontal);
        //left
        rs_piece(&mut moves, piece, 1, exit_horizontal);

        moves &= !own_mask;
        out.push(moves);
        pieces &= !piece;
        idx = pieces.trailing_zeros();
    }

    out
}

pub fn get_bishop_moves(bishops: u64, own_mask: u64, opp_mask: u64) -> Vec<u64> {
    let mut out = vec![];
    let mut pieces = bishops;
    let exit_mask = RANK_17 | FILE_A | FILE_H | opp_mask | own_mask;
    
    let mut idx = pieces.trailing_zeros();
    while idx != 64 {
        let mut moves = 0u64;
        let piece = 1u64 << idx;

        //up-right
        ls_piece(&mut moves, piece, 9, exit_mask);
        //down-right
        rs_piece(&mut moves, piece, 7, exit_mask);
        //up-left
        ls_piece(&mut moves, piece, 7, exit_mask);
        //down-left
        rs_piece(&mut moves, piece, 9, exit_mask);

        moves &= !own_mask;
        out.push(moves);
        pieces &= !piece;
        idx = pieces.trailing_zeros();
    }
    
    out
}

#[inline]
fn ls_piece(moves: &mut u64, piece: u64, shift: u32, exit_mask: u64) {
    let mut current_shift = shift;
    loop {
        let next = piece.wrapping_shl(current_shift);
        *moves |= next;
        if next & exit_mask != 0 {break;}
        current_shift += shift;
    }
}

#[inline]
fn rs_piece(moves: &mut u64, piece: u64, shift: u32, exit_mask: u64) {
    let mut current_shift = shift;
    loop {
        let next = piece.wrapping_shr(current_shift);
        *moves |= next;
        if next & exit_mask != 0 {break;}
        current_shift += shift;
    }
}