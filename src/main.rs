use std::io::{self, Write};

use bit_chess::*;

fn main() {
    let mut board = [
        WHITE_PAWNS,
        WHITE_KNIGHTS,
        WHITE_BISHOPS,
        WHITE_ROOKS,
        WHITE_QUEENS,
        WHITE_KING,
        
        BLACK_PAWNS,
        BLACK_KNIGHTS,
        BLACK_BISHOPS,
        BLACK_ROOKS,
        BLACK_QUEENS,
        BLACK_KING,
    ];
    
    let mut is_white = false;

    let mut can_o_o = true;
    
    let mut can_o_o_o = true;
    
    let mut en_passantable = 0u64;

    'main: loop {
        //print current board:
        print_table(&board);

        //get a unified mask for checking if something's in the way
        let uni_mask = get_unified_mask(&board);
        let opp_mask = get_unified_mask(&board[if is_white {6..12} else {0..6}]);
        let own_mask = get_unified_mask(&board[if is_white {0..6} else {6..12}]);

        //get the index for color
        let idx = if is_white {0} else {6};
        let opp_idx = if is_white {6} else {0};

        let checked = is_checked(
            board[idx + 5], 
            uni_mask, 
            board[opp_idx], 
            board[opp_idx + 1], 
            board[opp_idx + 2] | board[opp_idx + 4], 
            board[opp_idx + 3] | board[opp_idx + 4], 
            is_white
        );
        
        //get all moves
        let pawn_moves = get_pawn_moves(board[idx], opp_mask, uni_mask, if is_white {WHITE_PAWNS} else {BLACK_PAWNS}, is_white);
        let knight_moves = get_knight_moves(board[idx + 1], own_mask);
        let bishop_moves = get_bishop_moves(board[idx + 2], own_mask, opp_mask);
        let rook_moves = get_rook_moves(board[idx + 3], own_mask, opp_mask);
        let queen_moves = get_queen_moves(board[idx + 4], own_mask, opp_mask);
        let king_moves = get_king_moves(board[idx + 5], own_mask);

        let mut legal_moves = Vec::new();

        //filter out illegal pawn_moves
        validate_moves(&mut legal_moves, 0, &pawn_moves, &board, idx, opp_idx, opp_mask, is_white);
        validate_moves(&mut legal_moves, 1, &knight_moves, &board, idx, opp_idx, opp_mask, is_white);
        validate_moves(&mut legal_moves, 2, &bishop_moves, &board, idx, opp_idx, opp_mask, is_white);
        validate_moves(&mut legal_moves, 3, &rook_moves, &board, idx, opp_idx, opp_mask, is_white);
        validate_moves(&mut legal_moves, 4, &queen_moves, &board, idx, opp_idx, opp_mask, is_white);
        validate_moves(&mut legal_moves, 5, &[king_moves], &board, idx, opp_idx, opp_mask, is_white);

        //add en_passants
        if en_passantable != 0 {
            add_en_passants(&mut legal_moves, &board, board[idx], en_passantable, is_white, idx, opp_idx);
        }

        if legal_moves.is_empty() && checked {
            println!("checkmate, {} won!", if is_white {"black"} else {"white"});
            break 'main;
        } else if legal_moves.is_empty() {
            println!("stalemate, draw!");
            break 'main;
        }

        //get next move
        print!("{} moves: ", if is_white {"white"} else {"black"});
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error reading input");

        let mut input = input.trim().split(' ');

        //check if player wants to castle
        let first = input.next().unwrap();

        if first == "o-o" {
            if !can_o_o | checked {
                println!("cannot castle, try again!");
                continue 'main;
            }

            //check if safe to castle
            let king_pos = board[idx + 5];
            if check_slice(
                &[king_pos << 1, king_pos << 2],
                uni_mask, 
                board[opp_idx], 
                board[opp_idx + 1], 
                board[opp_idx + 2] | board[opp_idx + 4], 
                board[opp_idx + 3] | board[opp_idx + 4], 
                is_white
            ) {
                println!("cannot castle, try again!");
                continue 'main;
            }

            //check if something's in the way
            if uni_mask & if is_white {WHITE_KINGSIDE_MASK} else {BLACK_KINGSIDE_MASK} != 0 {
                println!("cannot castle, try again!");
                continue 'main;
            }

            //castling logic
            board[idx + 5] <<= 2;
            let castling_rook = board[idx + 3] & O_O_PIECES;
            board[idx + 3] &= !castling_rook;
            board[idx + 3] |= castling_rook >> 2;
            can_o_o = false;
            can_o_o_o = false;
            is_white = !is_white;
            continue 'main;
        }
        if first == "o-o-o" {
            if !can_o_o_o {
                println!("cannot castle, try again!");
                continue 'main;
            }

            //check if safe to castle
            let king_pos = board[idx + 5];
            if check_slice(
                &[king_pos >> 1, king_pos >> 2, king_pos >> 3],
                uni_mask, 
                board[opp_idx], 
                board[opp_idx + 1], 
                board[opp_idx + 2] | board[opp_idx + 4], 
                board[opp_idx + 3] | board[opp_idx + 4], 
                is_white
            ) {
                println!("cannot castle, try again!");
                continue 'main;
            }

            //check if something's in the way
            if uni_mask & if is_white {WHITE_QUEENSIDE_MASK} else {BLACK_QUEENSIDE_MASK} != 0 {
                println!("cannot castle, try again!");
                continue 'main;
            }

            //castling logic
            board[idx + 5] >>= 2;
            let castling_rook = board[idx + 3] & O_O_O_PIECES;
            board[idx + 3] &= !castling_rook;
            board[idx + 3] |= castling_rook << 3;
            can_o_o = false;
            can_o_o_o = false;
            is_white = !is_white;
            continue 'main;
        }

        //get piece name
        let piece_type = match first {
            "p" | "pawn" => 0,
            "n" | "knight" => 1,
            "b" | "bishop" => 2,
            "r" | "rook" => 3,
            "q" | "queen" => 4,
            "k" | "king" => 5,
            other => {
                println!("no piece named {other}, try again!");
                continue 'main;
            }
        };

        //get information about old position, create required masks
        let old = rank_and_file(input.next().unwrap());
        
        //get information about new positon
        let new = rank_and_file(input.next().unwrap());
        
        //first layer of filtering out invalid moves (does it go out of the board) or doesn't move the piece
        if old.0 > 7 || old.1 > 7 || new.0 > 7 || new.1 > 7 || (old.0 == new.0 && old.1 == new.1){
            println!("invalid move, try again!");
            continue 'main;
        }
        
        //creating masks from inputs
        let old_mask = 1u64 << old.0 * 8 + old.1;
        let new_mask = 1u64 << new.0 * 8 + new.1;
        
        //check if move is legal
        if !legal_moves.contains(&(piece_type, old_mask, new_mask)) {
            println!("not legal move, try again!");
            continue 'main;
        }

        //check if takes
        //also check if en passant happened
        let behind_mask = if is_white {new_mask >> 8} else {new_mask << 8};
        if piece_type == 0 && behind_mask & en_passantable != 0 {
            board[opp_idx] &= !en_passantable;
        }
        else if new_mask & opp_mask != 0 {
            for mut i in 0..6 {
                i += opp_idx;
                if  new_mask & board[i] != 0 {
                    board[i] &= !new_mask;
                    break;
                }
            }
        }

        //chack if still can castle
        if can_o_o && old_mask & O_O_PIECES != 0 {
            can_o_o = false;
        }
        if can_o_o_o && old_mask & O_O_O_PIECES != 0 {
            can_o_o_o = false;
        }

        //check if now black able to en_passant
        if piece_type == 0 && old_mask & PANW_STARTS != 0 && new_mask & RANK_3_OR_4 != 0 {
            en_passantable = new_mask;
        } else {
            en_passantable = 0;
        }

        let type_idx = idx + piece_type;
        board[type_idx] &= !old_mask;
        board[type_idx] |= new_mask;

        //flip next move
        is_white = !is_white;
    }
}