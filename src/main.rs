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
        let piece = input.next().unwrap();

        //get information about old position, create required masks
        let old = rank_and_file(input.next().unwrap());
        let old_mask = 1u64 << old.0 * 8 + old.1;
        let old_mask_rev = old_mask ^ u64::MAX;

        //get information about new positon
        let new = rank_and_file(input.next().unwrap());

        //first layer of filtering out invalid moves (does it go out of the board) or doesn't move the piece
        if old.0 > 7 || old.1 > 7 || new.0 > 7 || new.1 > 7 || (old.0 == new.0 && old.1 == new.1){
            println!("invalid move, try again!");
            continue 'main;
        }

        //second layer of filtering out invalid moves (does it follow the rules)
        if match piece {
            "p" | "pawn" => {
                let delta_rank = new.0 - old.0;
                let delta_file = new.1 - old.1;

                delta_rank.abs() > 2 || //moves more than 2 squares vertically
                delta_file.abs() > 1 || //moves more than 2 squares vertically
                (delta_rank == 2 && delta_file == 0 && is_white && old_mask & WHITE_PAWNS == 0) || //moves 2 with white
                (delta_rank == -2 && delta_file == 0 && !is_white && old_mask & BLACK_PAWNS == 0) || //moves 2 with black
                (delta_rank == 1 && delta_file.abs() == 1 && is_white && old_mask << (8 + delta_file) == 0) || //takes with white
                (delta_rank == 1 && delta_file.abs() == 1 && !is_white && old_mask >> (8 + delta_file) == 0) //takes with black
            },
            "k" | "knight" => {
                (old.0 - new.0).abs() + (old.1 - new.1).abs() != 3 || (new.0 == 0 || new.1 == 0) 
            },
            "b" | "bishop" => {
                old.0 - old.1 != new.0 - new.1 &&
                old.0 + old.1 != new.0 + new.1
            }"r" | "rook" => {
                old.0 != new.0 && old.1 != new.1
            },
            "q" | "queen" => {
                old.0 != new.0 && old.1 != new.1 &&
                (old.0 - old.1 != new.0 - new.1 && old.0 + old.1 != new.0 + new.1)
            },
            "king" => {
                (old.0 - new.0).abs() > 1 || (old.1 - new.1).abs() > 1
            },
            other => {
                println!("no piece called \"{other}\"");
                continue 'main;
            }
        } {
            println!("invalid move, try again!");
            continue 'main;
        }

        //create necesarry mask about the new position
        let new_mask = 1u64 << new.0 * 8 + new.1;

        //get the index of the given piece in the board array
        let mut idx: usize = if is_white {0} else {6};

        //create an int to create map for "travelled squares" (excluding squares which the given piece can take)
        let mut squares: u64 = 0;

        //exit early if there is none of the given piece in the given position
        //construct the traveled squares mask
        match piece {
            "p" | "pawn" => {
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}

                if old.1 == new.1 {
                    if is_white {
                        for rank in old.0 + 1..=new.0 {
                            squares |= 1u64 << rank * 8 + new.1;
                        };
                    } else {
                        for rank in new.0..old.0 {
                            squares |= 1u64 << rank * 8 + new.1;
                        }
                    }
                }
            },
            "k" | "knight" => {
                idx += 1;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}
                //knigh can't pass through anything therefore no "travelled squares" mask needed
            },
            "b" | "bishop" => {
                idx += 2;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}

                //file distance is equal to rank distance, doesn't matter which one we check
                let file_distance = (old.0 - new.0).abs();
                let rank_step = if old.0 > new.0 {-1} else {1};
                let file_step = if old.1 > new.1 {-1} else {1};
                for i in 1..file_distance {
                    let rank = old.0 + i * rank_step;
                    let file = old.1 + i * file_step;
                    squares |= 1u64 << rank * 8 + file;
                }
            },
            "r" | "rook" => {
                idx += 3;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}

                if old.0 != new.0 { // change within a file 
                    let rank_distance = (old.0 - new.0).abs();
                    let step = if old.0 > new.0 {-1} else {1};
                    for i in 1..rank_distance {
                        squares |= 1u64 << (old.0 + i * step) * 8 + old.1;
                    }
                } else { // change is within a rank
                    let file_distance = (old.1 - new.1).abs();
                    let step = if old.1 > new.1 {-1} else {1};
                    for i in 1..file_distance {
                        squares |= 1u64 << old.0 * 8 + old.1 + i * step;
                    }
                }
            },
            "q" | "queen" => {
                idx += 4;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}
                if old.0 != new.0 && old.1 != new.1 { // moved diagnal
                    //file distance is equal to rank distance, doesn't matter which one we check
                    let file_distance = (old.0 - new.0).abs();
                    let rank_step = if old.0 > new.0 {-1} else {1};
                    let file_step = if old.1 > new.1 {-1} else {1};
                    for i in 1..file_distance {
                        let rank = old.0 + i * rank_step;
                        let file = old.1 + i * file_step;
                        squares |= 1u64 << rank * 8 + file;
                    }
                }
                else if old.0 != new.0 { // change within a file
                    let rank_distance = (old.0 - new.0).abs();
                    let step = if old.0 > new.0 {-1} else {1};
                    for i in 1..rank_distance {
                        squares |= 1u64 << (old.0 + i * step) * 8 + old.1;
                    }
                } else { // change is within a rank
                    let file_distance = (old.1 - new.1).abs();
                    let step = if old.1 > new.1 {-1} else {1};
                    for i in 1..file_distance {
                        squares |= 1u64 << old.0 * 8 + old.1 + i * step;
                    }
                }
            },
            "king" => {
                idx += 5;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}
                squares = new_mask & own_mask;
            },
            _ => ()
        }

        //check if the piece doesn't pass through anything;
        if uni_mask & squares != 0 {println!("something's in the way, try again!"); continue 'main;}

        //check if takes something
        if new_mask & opp_mask != 0 {
            let opp_pieces = if is_white {&mut board[6..12]} else {&mut board[0..6]};
            for piece in opp_pieces {
                let taken_piece = new_mask & *piece;
                if taken_piece != 0 {
                    *piece &= taken_piece ^ u64::MAX;
                    break;
                }
            }
        }

        //move the piece
        board[idx] &= old_mask_rev;
        board[idx] |= new_mask;

        //flip next move
        is_white = !is_white;
    }
}