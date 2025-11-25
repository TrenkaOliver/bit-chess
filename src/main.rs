use std::io::{self, Write};

use bit_chess::*;

const WHITE_PAWNS: u64   = 0b1111_1111_u64 << 8;
const WHITE_KNIGHTS: u64 = 0b0100_0010_u64;
const WHITE_BISHOPS: u64 = 0b0010_0100_u64;
const WHITE_ROOKS: u64   = 0b1000_0001_u64;
const WHITE_QUEENS: u64  = 0b0000_1000_u64;
const WHITE_KING: u64    = 0b0001_0000_u64;

const BLACK_PAWNS: u64   = WHITE_PAWNS << 40;
const BLACK_KNIGHTS: u64 = WHITE_KNIGHTS << 56;
const BLACK_BISHOPS: u64 = WHITE_BISHOPS << 56;
const BLACK_ROOKS: u64   = WHITE_ROOKS << 56;
const BLACK_QUEENS: u64  = WHITE_QUEENS << 56;
const BLACK_KING: u64    = WHITE_KING << 56;

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

        //get all legal moves for each piece type:
        let pawn_moves = get_pawn_moves(board[idx], opp_mask, uni_mask, if is_white {WHITE_PAWNS} else {BLACK_PAWNS}, is_white);
        let knight_moves = get_knight_moves(board[idx + 1], own_mask);
        let bishop_moves = get_bishop_moves(board[idx + 2], own_mask, opp_mask);
        let rook_moves = get_rook_moves(board[idx + 3], own_mask, opp_mask);
        let queen_moves = get_queen_moves(board[idx + 4], own_mask, opp_mask);

        


        let checked = is_in_check(
            board[if is_white {5} else {11}],
            uni_mask,
            board[opp_idx],
            board[opp_idx + 1], 
            board[opp_idx + 2] | board[opp_idx + 4],
            board[opp_idx + 3] | board[opp_idx + 4],
            is_white,
        );

        println!("in check: {}", checked);


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
                //todo: implement
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