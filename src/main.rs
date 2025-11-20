use std::io::{self, Write};

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
    
    let mut is_next_white = true;

    'main: loop {
        //print current board:
        for rank in (0..8).rev() {
            let rank_value = rank * 8;
            print!("{} ", rank + 1);
            for file in 0..8 {
                let square = 1u64 << (rank_value + file);
                if board[0] & square != 0 {
                    print!("♙ ");
                } else if board[6] & square != 0 {
                    print!("♟ ");
                } else if board[1] & square != 0 {
                    print!("♘ ");
                } else if board[7] & square != 0 {
                    print!("♞ ");
                } else if board[2] & square != 0 {
                    print!("♗ ");
                } else if board[8] & square != 0 {
                    print!("♝ ");
                } else if board[3] & square != 0 {
                    print!("♖ ");
                } else if board[9] & square != 0 {
                    print!("♜ ");
                } else if board[4] & square != 0 {
                    print!("♕ ");
                } else if board[10] & square != 0 {
                    print!("♛ ");
                } else if board[5] & square != 0 {
                    print!("♔ ");
                } else if board[11] & square != 0 {
                    print!("♚ ");
                } else {
                    print!("・");
                }
            }
            println!();
        }
        println!("  A B C D E F G H\n");

        //get next move
        print!("{} moves: ", if is_next_white {"white"} else {"black"});
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

        //first layer of filtering out invalid moves (does it go out of the board)
        if old.0 > 7 || old.1 > 7 || new.0 > 7 || new.1 > 7 || (old.0 == new.0 && old.1 == new.1){
            println!("invalid move, try again!");
            continue 'main;
        }

        //second layer of filtering out invalid moves (does it follow the rules)
        if match piece {
            "p" | "pawn" => {
                old.1 != new.1 || (new.0 as i8 - old.0 as i8).abs() > 2
            },
            "k" | "knight" => {
                (old.0 as i8 - new.0 as i8).abs() + (old.1 as i8 - new.1 as i8).abs() != 3
            },
            "b" | "bishop" => {
                (old.0 as i8 - old.1 as i8) != (new.0 as i8 - new.1 as i8) &&
                old.0 + old.1 != new.0 + new.1
            }"r" | "rook" => {
                old.0 != new.0 && old.1 != new.1
            },
            "q" | "queen" => {
                old.0 != new.0 && old.1 != new.1 &&
                ((old.0 as i8 - old.1 as i8) != (new.0 as i8 - new.1 as i8) && old.0 + old.1 != new.0 + new.1)
            },
            "king" => {
                (old.0 as i8 - new.0 as i8).abs() > 1 || (old.1 as i8 - new.1 as i8).abs() > 1
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
        let new_mask_rev = new_mask ^ u64::MAX;

        //get a unified mask for checking if something's in the way
        let mask_uni = get_unified_mask(&board);

        //get the index of the given piece in the board array
        let mut idx: usize = if is_next_white {0} else {6};

        //create an int to create map for "travelled squares"
        let mut squares: u64 = 0;

        //exit early if there is none of the given piece in the given position
        //construct the traveled squares mask
        match piece {
            "p" | "pawn" => {
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}
                if is_next_white {
                    for rank in old.0 + 1..=new.0 {
                        squares |= 1u64 << rank * 8 + new.1;
                    };
                } else {
                    for rank in new.0..old.0 {
                        squares |= 1u64 << rank * 8 + new.1;
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
                //todo: implement
            },
            "r" | "rook" => {
                idx += 3;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}
                //todo: implement
            },
            "q" | "queen" => {
                idx += 4;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}
                //todo: implement
            },
            "king" => {
                idx += 5;
                if old_mask & board[idx] == 0 {println!("invalid move, try again!"); continue 'main;}
                //todo: implement                
            },
            _ => ()
        }

        //check if the piece doesn't pass through anything;
        if mask_uni & squares != 0 {println!("something's in the way..."); continue 'main;}

        //move the piece
        board[idx] &= old_mask_rev;
        board[idx] |= new_mask;

        //flip next move
        is_next_white = !is_next_white;
    }
}

//0: rank, 1: file
fn rank_and_file(input: &str) -> (u8, u8) {
    let mut chars = input.chars();
    let file = chars.next().unwrap().to_ascii_uppercase() as u8;
    let rank = chars.next().unwrap() as u8;
    (
        rank - 49, //rank
        file - 65, //file
    )
}

fn get_unified_mask(pieces: &[u64]) -> u64 {
    let mut result: u64 = 0;
    for piece in pieces.iter() {
        result |= piece;
    }
    result
}