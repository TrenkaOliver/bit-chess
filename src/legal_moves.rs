use crate::*;

//in all case (u64, u64) = (current_pos, legal_moves_mask)

pub fn get_king_moves(king: u64, own_mask: u64) -> (u64, u64) {
    let king_not_a = king & NOT_FILE_A;
    let king_not_h = king & NOT_FILE_H;
    let king_not_rank_0 = king & NOT_RANK_0;
    let king_not_rank_7 = king & NOT_RANK_7;

    (
        king,
        (king_not_a >> 1 |
        king_not_h << 1 |
        king_not_rank_0 >> 8 |
        king_not_rank_7 << 8 |
        (king_not_a & king_not_rank_0) >> 9 |
        (king_not_a & king_not_rank_7) << 7 |
        (king_not_h & king_not_rank_0) >> 7 |
        (king_not_h & king_not_rank_7) << 9 )
        & !own_mask
    )
}

pub fn get_pawn_moves(pawns: u64, opp_mask: u64, uni_mask: u64, start_pos: u64, is_white: bool) -> Vec<(u64, u64)> {
    let mut out = vec![];
    let mut pieces = pawns;
    let mut idx = pieces.trailing_zeros();

    while idx != 64 {
        let piece = 1u64 << idx;
        let mask =  if is_white {
            piece << 8 & !uni_mask |
            ((piece & start_pos) << 16) & !(uni_mask | uni_mask << 8) |
            piece & (opp_mask >> 7 | opp_mask >> 9) 
        } else {
            piece >> 8 & !uni_mask |
            ((piece & start_pos) >> 16) & !(uni_mask | uni_mask >> 8) |
            piece & (opp_mask << 7 | opp_mask << 9)
        };

        out.push((piece, mask));
        pieces &= !piece;
        idx = pieces.trailing_zeros();
    }
    
    out
}

pub fn get_knight_moves(knights: u64, own_mask: u64) -> Vec<(u64, u64)> {
    let mut out = vec![];
    let mut pieces = knights;
    let mut idx = pieces.trailing_zeros();

    while idx != 64 {
        let piece = 1u64 << idx;

        let knight_not_a = NOT_FILE_A & piece;
        let knight_not_ab = NOT_FILE_AB & piece;
        let knight_not_gh = NOT_FILE_GH & piece;
        let knight_not_h = NOT_FILE_H & piece;

        let mask = (knight_not_a << 15 |
        knight_not_a >> 17 |
        knight_not_ab << 6 |
        knight_not_ab >> 10 |
        knight_not_gh << 10 |
        knight_not_gh >> 6 |
        knight_not_h << 17 |
        knight_not_h >> 15) 
        & !own_mask;
        
        out.push((piece, mask));
        pieces &= !piece;
        idx = pieces.trailing_zeros();
    }

    out

}

pub fn get_rook_moves(rooks: u64, own_mask: u64, opp_mask: u64) -> Vec<(u64, u64)> {
    let mut out = vec![];
    let mut pieces = rooks;
    let exit_vertical = RANK_1_OR_7 | opp_mask | own_mask;
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
        out.push((piece, moves));
        pieces &= !piece;
        idx = pieces.trailing_zeros();
    }

    out
}

pub fn get_bishop_moves(bishops: u64, own_mask: u64, opp_mask: u64) -> Vec<(u64, u64)> {
    let mut out = vec![];
    let mut pieces = bishops;
    let exit_mask = RANK_1_OR_7 | FILE_A | FILE_H | opp_mask | own_mask;
    
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
        out.push((piece, moves));
        pieces &= !piece;
        idx = pieces.trailing_zeros();
    }
    
    out
}

pub fn get_queen_moves(queens: u64, own_mask: u64, opp_mask: u64) -> Vec<(u64, u64)> {
    let rook_like = get_rook_moves(queens, own_mask, opp_mask);
    let bishop_like = get_bishop_moves(queens, own_mask, opp_mask);

    debug_assert_eq!(rook_like.len(), bishop_like.len());

    rook_like.iter().zip(bishop_like.iter()).map(|((pos, a), (_, b))| (*pos, (a | b))).collect()
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