use std::io::{self, Write};

fn main() {
    let mut white_pawns = 0b1111_1111_u64 << 8;
    let mut white_knights = 0b0100_0010_u64;
    let mut white_bishops = 0b0010_0100_u64;
    let mut white_rooks = 0b1000_0001_u64;
    let mut white_queen = 0b0000_1000_u64;
    let mut white_king = 0b0001_0000_u64;

    let mut black_pawns = white_pawns << 40;
    let mut black_knights = white_knights << 56;
    let mut black_bishops = white_bishops << 56;
    let mut black_rooks = white_rooks << 56;
    let mut black_queen = white_king << 56;
    let mut black_king = white_queen << 56;

    let mut is_next_white = true;

    'main: loop {
        //print current board:
        for rank in (0..8).rev() {
            let rank_value = rank * 8;
            print!("{} ", rank + 1);
            for file in 0..8 {
                let square = 1u64 << (rank_value + file);
                if white_pawns & square != 0 {
                    print!("♙");
                } else if black_pawns & square != 0 {
                    print!("♟");
                } else if white_knights & square != 0 {
                    print!("♘");
                } else if black_knights & square != 0 {
                    print!("♞");
                } else if white_bishops & square != 0 {
                    print!("♗");
                } else if black_bishops & square != 0 {
                    print!("♝");
                } else if white_rooks & square != 0 {
                    print!("♖");
                } else if black_rooks & square != 0 {
                    print!("♜");
                } else if white_queen & square != 0 {
                    print!("♕");
                } else if black_queen & square != 0 {
                    print!("♛");
                } else if white_king & square != 0 {
                    print!("♔");
                } else if black_king & square != 0 {
                    print!("♚");
                } else {
                    print!("·");
                }
                print!(" ");
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
        let piece = input.next().unwrap();
        let old = rank_and_file(input.next().unwrap());
        let old_mask = 1u64 << old.0 * 8 + old.1;
        let new = rank_and_file(input.next().unwrap());
        let new_mask = 1u64 << new.0 * 8 + new.1;

        assert_eq!(piece, "pawn");
        assert!(is_next_white);

        //check if there's a piece to move:
        if old_mask & white_pawns != 0 {
            println!("old pos is valid");
            //check if there's a piece in the way:
            let black_mask = black_pawns | black_bishops | black_knights | black_rooks | black_queen | black_king; 
            let mut white_mask = 0;
            for rank in old.0..=new.0 {
                let rank_shift = rank * 8;
                for file in old.1..=new.1 {
                    white_mask |= 1u64 << rank_shift + file;
                }
            }
            if black_mask & white_mask != 0 { 
                println!("can't make this move, try again"); 
                continue 'main;
            }

            //actually move the piece:
            white_pawns &= old_mask ^ u64::MAX;
            white_pawns |= new_mask;
        }

        //flip next move
        is_next_white = !is_next_white;
    }
}

//0: rank, 1: file
pub fn rank_and_file(input: &str) -> (u8, u8) {
    let mut chars = input.chars();
    let file = chars.next().unwrap().to_ascii_uppercase() as u8;
    let rank = chars.next().unwrap() as u8;
    (
        rank - 49, //rank
        file - 65, //file
    )
}