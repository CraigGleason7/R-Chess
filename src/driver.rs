
static MOVE_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"([a-hA-H])([1-8]).*([a-hA-H])([1-8])").unwrap() 
});

// macro to check if all coordinates are within the bounds of the board
macro_rules! in_bounds {
    ( $( $val:expr ),+ ) => {
        if $( (($val) as i8) < 0 || (($val) as i8) > 7 )||+ {
            false
        }
        else {
            true
        }
    };
}

macro_rules! delta {
    ($val1:expr, $val2:expr) => {
        (($val2) as i8 - ($val1) as i8).abs()
    };
}

// macro to invert direction of arthmetic if color is Black (i.e. a pawn needs to increase its y value if its White, and decrease if its Black)
macro_rules! apply_direction {
    ($body:expr, $color:expr) => {
        match $color {
            PieceColor::White => { ($body) }
            PieceColor::Black => { 0 - ($body)}
        }
    };
}
    
// macro to get increment for interpolation depending on 
macro_rules! get_increment {
    ($initial:expr, $final:expr) => {

        if ($final > $initial) {
            1_i8
        }
        else if ($final == $initial) {
            0_i8
        }
        else {
            -1_i8
        }   
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub enum GameState {
    Playing,
    Checkmate,
    Stalemate
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece {
    color: PieceColor,
    piece_type: PieceType,
    has_moved: bool,
    en_passantable: bool
}

pub fn parse_move(input: &str) -> Option<(usize, usize, usize, usize)> {

    let input_lower = input.to_lowercase();
    let caps = MOVE_REGEX.captures(input_lower.as_str())?;

    let x_i = (caps[1].chars().next()? as u32 - 'a' as u32) as usize;
    let y_i = (caps[2].chars().next()? as u32 - '1' as u32) as usize;
    let x_f = (caps[3].chars().next()? as u32 - 'a' as u32) as usize;
    let y_f = (caps[4].chars().next()? as u32 - '1' as u32) as usize;

    Some((x_i, y_i, x_f, y_f))
}

pub fn init_board(board: &mut [[Option<Piece>; 8]; 8]) {

    const HOME_ROW: [PieceType; 8] = [
        PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen, 
        PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook
    ];

    const WHITE_HOMEROW_NUM: usize = 0;
    const WHITE_PAWNROW_NUM: usize = 1;

    const BLACK_HOMEROW_NUM: usize = 7;
    const BLACK_PAWNROW_NUM: usize = 6;

    for (row_num, row) in board.iter_mut().enumerate() {
        for (column_num, square) in row.iter_mut().enumerate() {

            *square = match row_num {

                WHITE_HOMEROW_NUM => 
                    Some(Piece { color: PieceColor::White, piece_type: HOME_ROW[column_num], has_moved: false, en_passantable: false}), 
                    
                WHITE_PAWNROW_NUM => 
                    Some(Piece { color: PieceColor::White, piece_type: PieceType::Pawn, has_moved: false, en_passantable: false}),
                    
                BLACK_PAWNROW_NUM => 
                    Some(Piece { color: PieceColor::Black, piece_type: PieceType::Pawn, has_moved: false, en_passantable: false}),
                    
                BLACK_HOMEROW_NUM => 
                    Some(Piece { color: PieceColor::Black, piece_type: HOME_ROW[column_num], has_moved: false, en_passantable: false}), 
                    
                _ => 
                    None
            }

        }
    }
}

pub fn print_board(board: &[[Option<Piece>; 8]; 8]) {

    println!("   A B C D E F G H"); 
    println!("  +-----------------+");

    for (y, row) in board.iter().enumerate().rev() {
            
        print!("{} | ", y + 1);

        for square in row.iter() {
            match square {
                Some(piece) => {
                        
                    let c = match piece.piece_type {
                        PieceType::Pawn => 'p',
                        PieceType::Knight => 'n',
                        PieceType::Bishop => 'b',
                        PieceType::Rook => 'r',
                        PieceType::Queen => 'q',
                        PieceType::King => 'k',
                    };

                    // Logic: Uppercase for Black, Lowercase for White
                    if piece.color == PieceColor::Black {
                        print!("{} ", c.to_ascii_uppercase());
                    } else {
                        print!("{} ", c.to_ascii_lowercase());
                    }
                }
                // Print a dot for empty squares
                None => print!(". "),
            }
        }
        println!("| {}", y + 1);
    }

    println!("  +-----------------+");
    println!("   A B C D E F G H");
}

// directly moves piece. Assumes the move is valid. Updates struct fields as nessasary
pub fn move_piece(coords: (usize, usize, usize, usize), board: &mut [[Option<Piece>; 8]; 8]) {

    let (x, y, x2, y2) = coords;

    // sets board[y][x] as None automagically
    let mut piece = board[y][x].take().unwrap();

    //update struct fields
    piece.has_moved = true;
    if piece.piece_type == PieceType::Pawn && ((y2 as i8 - y as i8).abs() == 2) {
        piece.en_passantable = true;
    }

    // CASTLE LOGIC
    if (piece.piece_type == PieceType::King) && (delta!(x,x2) > 1) {

        // get rook and take ownership
        let mut rook = board[y][ if x2 > x { 7 } else { 0 }].take().unwrap();

        // update rook struct fields
        rook.has_moved = true;

        // move king and rook
        board[y2][ if x2 > x { x + 2 } else { x - 2 }] = Some(piece);
        board[y2][ if x2 > x { x + 1 } else { x - 1 }] = Some(rook);

    }
    // EN PASSANT LOGIC
    else if (piece.piece_type == PieceType::Pawn) && (delta!(x,x2) == 1) && board[y2][x2].is_none() {
        // delete captured pawn
        board[if piece.color == PieceColor::White {y2 - 1} else {y2 + 1}][x2] = None;
        // move pawn to new location
        board[y2][x2] = Some(piece); 
    }
    // pawn promotion logic (no underpromotion)
    else if piece.piece_type == PieceType::Pawn && y2 == 7 || y2 == 0 {
        board[y2][x2] = Some(Piece{color: piece.color, piece_type: PieceType::Queen, has_moved: true, en_passantable: false}); 
    }
    // normal move logic    
    else {
        // move piece to new location
        board[y2][x2] = Some(piece); 
    }
}

pub fn is_move_valid(coords: (usize, usize, usize, usize), turn: PieceColor, board: &[[Option<Piece>; 8]; 8]) -> bool {

    let (x, y, x2, y2) = coords;

    // idiot check
    if !in_bounds!(x, y, x2, y2) || ((x == x2) && (y == y2)) || board[y][x].is_none() {
        return false
    }

    // grab reference to piece
    let piece = board[y][x].as_ref().unwrap();

    // make sure it's that piece's turn
    if piece.color != turn {
        return false
    }

    // this is where the individual move logic is handled
    let move_work = match piece.piece_type {
        PieceType::Pawn => is_pawn_move_valid(coords, piece, board),
        PieceType::Knight => is_knight_move_valid(coords, piece, board),
        PieceType::Bishop => is_bishop_move_valid(coords, piece, board),
        PieceType::Rook => is_rook_move_valid(coords, piece, board),
        PieceType::Queen => is_queen_move_valid(coords, piece, board),
        PieceType::King => is_king_move_valid(coords, piece, board)
    };

    if !move_work {
        return false
    }

    // copy board
    let mut temp_board = *board;

    // attempt move to calculate check
    move_piece(coords, &mut temp_board);

    // get king's position
    let king_pos = find_king(turn, &temp_board);

    // test if the new board state will place king in check
    if check_check(king_pos, turn, &temp_board) {
        return false
    }

    // if all passes, return true
    true
}

fn is_pawn_move_valid(coords: (usize, usize, usize, usize), piece: &Piece, board: &[[Option<Piece>; 8]; 8]) -> bool {

    // signed integer instead of unsigned in order to not have overflow when doing arithmetic 
    let (x, y, x2, y2) = (coords.0 as i8, coords.1 as i8, coords.2 as i8, coords.3 as i8);

    // match based on if the pawn is moving or capturing
    match delta!(x,x2) {

        // if pawn is not capturing
        0 => {  
            // if there's something where pawn is trying to move, can't move there
            return match apply_direction!(y2 - y, piece.color) {
                1 => board[y2 as usize][x2 as usize].is_none(),
                2 => !piece.has_moved && board[y2 as usize][x2 as usize].is_none() && board[(if y2 > y {y + 1} else {y - 1}) as usize][x2 as usize].is_none(),
                _ => false
            } 
        }

        // if pawn is capturing
        1 => {
            // pawn must move one square forward
            if (apply_direction!(y2 - y, piece.color) != 1) {
                return false;
            }
            // check final location
            match board[y2 as usize][x2 as usize] {
                // if there is a piece there
                Some(captured_piece) => captured_piece.color != piece.color,
                // if there is no piece there
                None => {   
                    // check en passant
                    if let Some(captured_piece) = board[y as usize][x2 as usize] {
                        // return true if its different color, and it is en passantable
                        return (captured_piece.color != piece.color) && captured_piece.en_passantable;
                    }
                    // if there is nothing next to pawn, no en passant
                    else {
                        false
                    }

                }
            }
        }

        // illegal
        _ => {
            false
        }
    }
}

fn is_knight_move_valid(coords: (usize, usize, usize, usize), piece: &Piece, board: &[[Option<Piece>; 8]; 8]) -> bool {
    
    // signed integer instead of unsigned in order to not have overflow when doing arithmetic 
    let (x, y, x2, y2) = (coords.0 as i8, coords.1 as i8, coords.2 as i8, coords.3 as i8);

    // makes sure that the changes in x and y add up to 3, and the difference between them is 1 (meaning one difference much be 1 and the other must be 2)
    if ((delta!(x,x2) + delta!(y,y2)) != 3) || (delta!(delta!(x,x2),delta!(y,y2)) != 1) {
        return false
    };

    // make sure ending square has different color as piece being moved
    if board[y2 as usize][x2 as usize].is_some() && (board[y2 as usize][x2 as usize].as_ref().unwrap().color == piece.color) {
        return false
    }

    true
}

fn is_bishop_move_valid(coords: (usize, usize, usize, usize), piece: &Piece, board: &[[Option<Piece>; 8]; 8]) -> bool {

    let (x, y, x2, y2) = (coords.0 as i8, coords.1 as i8, coords.2 as i8, coords.3 as i8);

    // checks to make sure it's moving diagonally, and that there's nothing in the way
    (delta!(x,x2) == delta!(y,y2)) && interpolate_move(coords, piece, board)
}

fn is_rook_move_valid(coords: (usize, usize, usize, usize), piece: &Piece, board: &[[Option<Piece>; 8]; 8]) -> bool {

    let (x, y, x2, y2) = (coords.0 as i8, coords.1 as i8, coords.2 as i8, coords.3 as i8);

    // checks to make sure it's moving horizontally, and that there's nothing in the way
    ((delta!(x,x2) == 0) || (delta!(y,y2) == 0)) && interpolate_move(coords, piece, board)
}

fn is_queen_move_valid(coords: (usize, usize, usize, usize), piece: &Piece, board: &[[Option<Piece>; 8]; 8]) -> bool {
    // lol
    is_bishop_move_valid(coords, piece, board) || is_rook_move_valid(coords, piece, board)
}

fn is_king_move_valid(coords: (usize, usize, usize, usize), piece: &Piece, board: &[[Option<Piece>; 8]; 8]) -> bool {

    let (x, y, x2, y2) = (coords.0 as i8, coords.1 as i8, coords.2 as i8, coords.3 as i8);

    // if king only moving one space, only make sure that theres nothing in the way on the other side and that it wouldn't be in check in that location
    if (delta!(x, x2) <= 1) && (delta!(y, y2) <= 1) {
        return interpolate_move(coords, piece, board) && !check_check((coords.2, coords.3), piece.color, board)
    }

    // castling logic (dont need to check y value since has_moved would be false if it wasn't home row)
    else if !piece.has_moved && (delta!(y,y2) == 0) {   
        // make sure theres a rook in the corner
        if let Some(corner_rook) = &board[y as usize][if x2 > x { 7 } else { 0 }] {

            // make sure king isn't moving thorugh check
            for x_iterator in if x2 > x {x..=x+2} else {x-2..=x} {
                if check_check((x_iterator as usize, y2 as usize), piece.color, board) {
                    return false
                }
            }

            // make sure nothing is in the way
            for x_iterator in if x2 > x {x+1..=6} else {1..=x-1} {
                if board[y2 as usize][x_iterator as usize].is_some() {
                    return false
                }
            }
            
            return corner_rook.color == piece.color && !corner_rook.has_moved                                                                                
        }
    }
    // if king has moved or is not being moved to same row,
    false   
}

// helper function to make sure everything in a line is clear, then checks to make sure piece final spot isn't same color as piece being moved
fn interpolate_move(coords: (usize, usize, usize, usize), piece: &Piece, board: &[[Option<Piece>; 8]; 8]) -> bool {

    let (mut x, mut y, x2, y2) = (coords.0 as i8, coords.1 as i8, coords.2 as i8, coords.3 as i8);

    let (x_increment, y_increment) = (get_increment!(x, x2), get_increment!(y, y2));

    // increment first to avoid checking starting square
    x += x_increment;
    y += y_increment;

    // iterates over all squares between starting and ending coordinants, making sure they are empty
    while (x != x2) || (y != y2) {

        if board[y as usize][x as usize].is_some() {
            return false;
        }

        x += x_increment;
        y += y_increment;
    }

    // make sure ending square has different color as piece being moved
    if board[y2 as usize][x2 as usize].is_some() && (board[y2 as usize][x2 as usize].as_ref().unwrap().color == piece.color) {
        return false
    }

    true
}

// helper function txo iterate down a line and return the first piece found in that line
fn get_first_piece(pos: (usize, usize), heading: (i8, i8), board: &[[Option<Piece>; 8]; 8]) -> Option<&Piece> {

    let (mut x, mut y, x_increment, y_increment) = (pos.0 as i8, pos.1 as i8, heading.0, heading.1);

    // handle no direction case
    if x_increment == 0 && y_increment == 0 {
        return None
    }

    // increment first to avoid checking starting square
    x += x_increment;
    y += y_increment;

    // iterates over all squares between starting and ending coordinants, making sure they are empty
    while in_bounds!(x,y) {

        if board[y as usize][x as usize].is_some() {
            return board[y as usize][x as usize].as_ref();
        }

        x += x_increment;
        y += y_increment;
    }

     None
} 

// helper function to check if the position is in check
pub fn check_check(pos: (usize, usize), color: PieceColor, board: &[[Option<Piece>; 8]; 8]) -> bool {

    let (x, y) = (pos.0 as i8, pos.1 as i8);

    // iterate over all rays 
    for y_increment in -1..=1 {
        for x_increment in -1..=1 {

            // if there exists a piece down that ray
            if let Some(attacker) = get_first_piece(pos, (x_increment, y_increment), board) {
                if attacker.color != color &&                                                               // if its a different color AND
                ((attacker.piece_type == PieceType::Rook && delta!(x_increment,y_increment) == 1) ||        // (its a rook and heading not diagonally OR
                 (attacker.piece_type == PieceType::Bishop && delta!(x_increment,y_increment) != 1) ||      // its a bishop and heading diagonally OR
                 (attacker.piece_type == PieceType::Queen)) {                                               // its a queen)
                    return true                                                                             // then we are in check
                }
            }

            // king check
            if in_bounds!(x + x_increment, y + y_increment) {
                if let Some(attacker) = board[(y + y_increment) as usize][(x + x_increment) as usize].as_ref() {
                    if attacker.color != color && attacker.piece_type == PieceType::King {
                        return true
                    }
                }
            }
        }
    }

    // knight logic
    for y_increment in -2_i8..=2 {
        for x_increment in -2_i8..=2 {
            // skip over the iterations where one direction is 0 or if the increments are equal, or if its not in bounds
            if y_increment == 0 || x_increment == 0 || y_increment.abs() == x_increment.abs() || !in_bounds!(x + x_increment,y + y_increment){
                continue;
            }

            // if there's a knight there, and its the opposite color, this square is in check
            if let Some(attacker) = board[(y + y_increment) as usize][(x + x_increment) as usize].as_ref() {
                if attacker.piece_type == PieceType::Knight && attacker.color != color {
                    return true
                }
            }
        }
    }

    // pawn logic
    for x_dir in [-1_i8, 1] {
        // if the possible attacking square is in bounds
        if in_bounds!(x + x_dir, y + apply_direction!(1,color)) {
            // if attacker square has a piece
            if let Some(attacker) = board[(y  + apply_direction!(1,color)) as usize][(x + x_dir) as usize].as_ref() {
                // if the piece is a pawn and a different color 
                if attacker.color != color && attacker.piece_type == PieceType::Pawn {
                    return true
                }
            }
        }
    }
    
    false
}

pub fn determine_game_state(turn: PieceColor, board: &[[Option<Piece>; 8]; 8]) -> GameState {

    // iterate over whole board
    for y in 0_usize..=7 {
        for x in 0_usize..=7 {

            // if no piece there, or piece is not correct color, continue loop
            if board[y][x].is_none() || board[y][x].as_ref().unwrap().color != turn {
                continue;
            }

            // if there is an available move, return normal gamestate
            if !get_available_moves((x,y), board).is_empty() {
                return GameState::Playing
            }    
        }
    }

    // if there is no available move, determine if in check, if so then checkmate else stalemate
    match check_check(find_king(turn, board), turn, board) {
        true => GameState::Checkmate,
        false => GameState::Stalemate
    }
}

// this is not the most effecient way to do this, should filter by piece type. this is just the simplest way
pub fn get_available_moves(pos: (usize, usize), board: &[[Option<Piece>; 8]; 8]) -> Vec<(usize, usize)> {

    let (x, y) = pos;

    let mut move_list: Vec<(usize, usize)> = vec![];

    let Some(piece) = &board[y][x] else {
        return move_list;
    };

    // iterate over whole board
    for y2 in 0_usize..=7 {
        for x2 in 0_usize..=7 {

            // if it's a valid move, add it to the return array
            if is_move_valid((x, y, x2, y2), piece.color, board) {
                move_list.push((x2, y2));
            }
        }
    }
    
    move_list
}

// called at every end of beginning of every turn, clears the ability to en passant the current colors pawns  
pub fn clear_en_passantable(turn: PieceColor, board: &mut [[Option<Piece>; 8]; 8]) {
    // iterates over every square, if its the same color as the turn and its a pawn, clear the ability to en passant it
    for row in board.iter_mut() {
        for square in row.iter_mut() {

            if let Some(piece) = square {
                if (piece.color == turn) && (piece.piece_type == PieceType::Pawn) {
                        piece.en_passantable = false;
                    }
            }
        }
    }
}

// find king, return (x,y) position
fn find_king(color: PieceColor, board: &[[Option<Piece>; 8]; 8]) -> (usize, usize) {
    // iterate through every square
    for (y, row) in board.iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color == color && piece.piece_type == PieceType::King {
                    return (x, y);
                }
            }
        }
    }

    // I'm so scared rn 
    panic!("Cannot find king");
}




