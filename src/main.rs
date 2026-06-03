mod driver;

use driver::PieceColor;
use driver::Piece;

use crate::driver::clear_en_passantable;
use crate::driver::is_move_valid;
use crate::driver::move_piece;


fn main() {

    print!("\x1B[2J\x1B[1;1H");
    println!("Shall we play a game?");
    println!("[type 'yes' to play chess]");

    let mut user_input = String::new();

    std::io::stdin()
            .read_line(&mut user_input)
            .expect("\nPeople sometimes make mistakes.\nYes, they do.");
    
    if user_input.trim() != "yes" {
        print!("\x1B[2J\x1B[1;1H"); 
        println!("Fine.");
        return;
    }

    let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    let mut turn: PieceColor = PieceColor::White;

    
    driver::init_board(&mut board);

    driver::print_board(&board);

    println!("\n[move format is 'A2 to A4']");

    loop {

        user_input.clear();

        println!("[make a move]");

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("\nPeople sometimes make mistakes.\nYes, they do.");

        if user_input.trim() == "exit" {
            break;
        }

        let parsed_move = driver::parse_move(&user_input);

        match parsed_move {

            Some(coords) => {
                
                if is_move_valid(coords, turn, &board) {
                    clear_en_passantable(turn, &mut board);
                    move_piece(coords, &mut board);
                }

            }

            None => {
                println!("\nPeople sometimes make mistakes.\nYes, they do.\n[invalid move input]");
                print!("\x1B[2J\x1B[1;1H");
            }
            
        }

        driver::print_board(&board);

        turn = match turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("A strange game. The only winning move is not to play. How about another game of chess?");
    println!("[Do not play another game of chess]");

    user_input.clear();
    std::io::stdin()
            .read_line(&mut user_input)
            .expect("\nPeople sometimes make mistakes.\nYes, they do.");
    
}




