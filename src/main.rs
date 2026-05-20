

#[derive(Debug, PartialEq, Copy, Clone)]
enum PieceColor {
    WHITE,
    BLACK
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Piece {
    color: PieceColor,
    piece: PieceType
}

fn init_board(board: &mut [[Option<Piece>; 8]; 8]) {

    const HOME_ROW: [PieceType; 8] = [
        PieceType::ROOK, PieceType::KNIGHT, PieceType::BISHOP, PieceType::QUEEN, 
        PieceType::KING, PieceType::BISHOP, PieceType::KNIGHT, PieceType::ROOK
    ];

    const WHITE_HOMEROW_NUM: usize = 0;
    const WHITE_PAWNROW_NUM: usize = 1;

    const BLACK_HOMEROW_NUM: usize = 7;
    const BLACK_PAWNROW_NUM: usize = 6;


    for (row_num, row) in board.iter_mut().enumerate() {
        for (column_num, square) in row.iter_mut().enumerate() {

            *square = match row_num {

                WHITE_HOMEROW_NUM => 
                    Some(Piece { color: PieceColor::WHITE, piece: HOME_ROW[column_num]}), 
                    
                WHITE_PAWNROW_NUM => 
                    Some(Piece { color: PieceColor::WHITE, piece: PieceType::PAWN}),
                    
                BLACK_PAWNROW_NUM => 
                    Some(Piece { color: PieceColor::BLACK, piece: PieceType::PAWN}),
                    
                BLACK_HOMEROW_NUM => 
                    Some(Piece { color: PieceColor::BLACK, piece: HOME_ROW[column_num]}), 
                    
                _ => 
                    None
            }

        }
    }
    

}

fn main() {
    println!("Hello, world!");

    let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    let mut turn: PieceColor = PieceColor::WHITE;

    init_board(&mut board);

    while (true) {


        match turn {
            PieceColor::WHITE
            turn = PieceColor::BLACK; }

    }

    println!("Hello, world!");
}




