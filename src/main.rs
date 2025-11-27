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
    
    let mut is_white = true;

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
            board[if is_white {5} else {11}],
            uni_mask,
            board[opp_idx],
            board[opp_idx + 1], 
            board[opp_idx + 2] | board[opp_idx + 4],
            board[opp_idx + 3] | board[opp_idx + 4],
            is_white,
        );

        println!("in check: {}", checked);

        if checked { 'block: {
            //detect mates
            //first see if king can move out:
            let temp_board = uni_mask & !board[idx + 5];
            let mut king_moves = get_king_moves(board[idx + 5], own_mask);
            let mut king_move_idx = king_moves.trailing_zeros();
            while king_move_idx != 64 {
                let new_king = 1u64 << king_move_idx;
                let new_board = temp_board | new_king;
                //check if king takes:
                let opp_pieces = if is_white {&mut board[6..12]} else {&mut board[0..6]};
                for piece in opp_pieces {
                    let taken_piece = new_board & *piece;
                    if taken_piece != 0 {
                        *piece &= !taken_piece;
                        break;
                    }
                }
                if !is_checked(
                    new_king,
                    new_board,
                    board[opp_idx],
                    board[opp_idx + 1], 
                    board[opp_idx + 2] | board[opp_idx + 4],
                    board[opp_idx + 3] | board[opp_idx + 4],
                    is_white,
                ) {
                    println!("exit 1");
                    break 'block;
                }
                king_moves &= !new_king;
                king_move_idx = king_moves.trailing_zeros();
            }

            //if couldn't move out with king move forward

            let (checking_piece, mut checking_path, checking_t) = get_check_mask(
                board[idx + 5],
                uni_mask,
                board[opp_idx],
                board[opp_idx + 1], 
                board[opp_idx + 2] | board[opp_idx + 4],
                board[opp_idx + 3] | board[opp_idx + 4],
                board[opp_idx + 4],
                is_white,
                false,
            ).unwrap(); //must be a check giving piece

            //check if the check giving piece can be taken:
            if let Some((taking_piece, _, taking_t)) = get_check_mask(
                checking_piece,
                uni_mask,
                board[idx],
                board[idx + 1], 
                board[idx + 2] | board[idx + 4],
                board[idx + 3] | board[idx + 4],
                board[idx + 4],
                is_white,
                false,
            ) {
                //take the piece on temp board
                let mut temp_board = board;
                temp_board[opp_idx + checking_t] &= !checking_piece;
                temp_board[idx + taking_t] &= !taking_piece;
                temp_board[idx + taking_t] |= checking_piece;
                let temp_uni_mask = get_unified_mask(&temp_board);

                //check if taking leaves the king exposed
                if !is_checked(
                    board[idx + 5],
                    temp_uni_mask,
                    temp_board[opp_idx],
                    temp_board[opp_idx + 1], 
                    temp_board[opp_idx + 2] | temp_board[opp_idx + 4],
                    temp_board[opp_idx + 3] | temp_board[opp_idx + 4],
                    is_white,
                ) {
                    println!("exit 2");
                    break 'block;
                }
            }

            //if can't take than check if able to block
            let mut square_to_block_idx = checking_path.trailing_zeros();
            while square_to_block_idx != 64 {
                let square_to_block = 1u64 << square_to_block_idx;
                if let Some((blocking_piece, _, blocking_t)) = get_check_mask(
                    square_to_block,
                    uni_mask,
                    board[idx],
                    board[idx + 1], 
                    board[idx + 2] | board[idx + 4],
                    board[idx + 3] | board[idx + 4],
                    board[idx + 4],
                    is_white,
                    true,
                ) {
                    //move the piece on temp board
                    let mut temp_board = board;
                    temp_board[idx + blocking_t] &= !blocking_piece;
                    temp_board[idx + blocking_t] |= square_to_block;
                    let temp_uni_mask = get_unified_mask(&temp_board);

                    //check if taking leaves the king exposed
                    if !is_checked(
                        board[idx + 5],
                        temp_uni_mask,
                        temp_board[opp_idx],
                        temp_board[opp_idx + 1], 
                        temp_board[opp_idx + 2] | temp_board[opp_idx + 4],
                        temp_board[opp_idx + 3] | temp_board[opp_idx + 4],
                        is_white,
                    ) {
                        println!("exit 3");
                        break 'block;
                    }
                }
                checking_path &= !square_to_block;
                square_to_block_idx = checking_path.trailing_zeros();
            }

            //if program gets here than mate:
            println!("mate, {} won", if is_white {"black"} else {"white"});
            break 'main;
        }}

        //get next move
        print!("{} moves: ", if is_white {"white"} else {"black"});
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error reading input");

        let mut input = input.trim().split(' '); 

        //get piece name
        let piece_type = input.next().unwrap();

        //get information about old position, create required masks
        let old = rank_and_file(input.next().unwrap());
        let old_mask = 1u64 << old.0 * 8 + old.1;

        //get information about new positon
        let new = rank_and_file(input.next().unwrap());
        let new_mask = 1u64 << new.0 * 8 + new.1;

        //first layer of filtering out invalid moves (does it go out of the board) or doesn't move the piece
        if old.0 < 0 || old.0 > 7 || old.1 < 0 || old.1 > 7 || new.0 < 0 || new.0 > 7 || new.1 < 0 || new.1 > 7 || (old.0 == new.0 && old.1 == new.1){
            println!("invalid move, try again!");
            continue 'main;
        }

        let mut piece_idx = idx;
        let legal_moves = match piece_type {
            "p" | "pawn" => {
                if old_mask & board[piece_idx] == 0 {
                    println!("no piece in the specifyed square, try again!");
                    continue 'main;
                }
                get_pawn_moves(old_mask, opp_mask, uni_mask, if is_white {WHITE_PAWNS} else {BLACK_PAWNS}, is_white)[0].1
            },
            "n" | "knight" => {
                piece_idx += 1;
                if old_mask & board[piece_idx] == 0 {
                    println!("no piece in the specifyed square, try again!");
                    continue 'main;
                }
                get_knight_moves(old_mask, own_mask)[0].1
            },
            "b" | "bishop" => {
                piece_idx += 2;
                if old_mask & board[piece_idx] == 0 {
                    println!("no piece in the specifyed square, try again!");
                    continue 'main;
                }
                get_bishop_moves(old_mask, own_mask, opp_mask)[0].1
            },
            "r" | "rook" => {
                piece_idx += 3;
                if old_mask & board[piece_idx] == 0 {
                    println!("no piece in the specifyed square, try again!");
                    continue 'main;
                }
                get_rook_moves(old_mask, own_mask, opp_mask)[0].1
            },
            "q" | "queen" => {
                piece_idx += 4;
                if old_mask & board[piece_idx] == 0 {
                    println!("no piece in the specifyed square, try again!");
                    continue 'main;
                }
                get_queen_moves(old_mask, own_mask, opp_mask)[0].1
            },
            "k" | "king" => {
                piece_idx += 5;
                if old_mask & board[piece_idx] == 0 {
                    println!("no piece in the specifyed square, try again!");
                    continue 'main;
                }
                get_king_moves(old_mask, own_mask)
            },
            other => {
                println!("no piece named {other}, try again!");
                continue 'main;
            }
        };

        print_mask(legal_moves);
        print_mask(new_mask);
        if new_mask & legal_moves == 0 {
            println!("cannot move here, try again!");
            continue 'main;
        }

        //check if takes
        let mut taken_from = 12; //need this to add back taken piece if move leaves king in check
        if new_mask & opp_mask != 0 {
            for mut i in 0..6 {
                i += opp_idx;
                if  new_mask & board[i] != 0 {
                    board[i] &= !new_mask;
                    taken_from = i;
                    break;
                }
            }
        }

        let mut temp_board = board;
        temp_board[piece_idx] &= !old_mask;
        temp_board[piece_idx] |= new_mask;

        //check if leaves king in check
        if is_checked(
            temp_board[if is_white {5} else {11}],
            get_unified_mask(&temp_board),
            temp_board[opp_idx],
            temp_board[opp_idx + 1], 
            temp_board[opp_idx + 2] | temp_board[opp_idx + 4],
            temp_board[opp_idx + 3] | temp_board[opp_idx + 4],
            is_white,
        ) {
            if taken_from != 12 {
                board[taken_from] |= new_mask;
            }
            println!("this move would leave the king in check, try again!");
            continue 'main;
        }

        board = temp_board;        

        //flip next move
        is_white = !is_white;
    }
}