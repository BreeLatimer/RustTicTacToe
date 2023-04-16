//tto.rs

fn print_board(board: [[u32; 3]; 3]) {
    // Prints the contents of the board. 1 is X, 2 is O, others blank.
    for row in board {
        let mut output = String::from("");
        for cell in row {
            if cell == 0 {
                output.push_str("[ ]");
            }
            else if cell == 1 {
                output.push_str("[X]");
            }
            else {
                output.push_str("[O]");
            }
        }
        println!("{}", output);
    }
}

fn keep_playing(board: [[u32; 3]; 3]) -> bool {
    // False if board full or either player's won, else true.
    
    if board_full(board) {
        return false;
    }
    else if get_winner(board) != 0 {
        return false;
    }
    else {
        return true;
    }
}

fn board_full(board: [[u32; 3]; 3]) -> bool {
    // True if board full, else false.
    for row in board {
        for cell in row {
            if cell == 0 {
                return false;
            }
        }
    }

    true // if we get to this point the board is full.
}

fn get_winner(board: [[u32; 3]; 3]) -> i32 {
    println!("Who won?");
    0  // 0 if none, 1 if Player, 2 if computer.
}

fn player_move(board: [[u32; 3]; 3]) {
      // Ask player for location on grid, then change to 1 (if not already taken).
}

fn computer_move(board: [[u32; 3]; 3]) {
     // Computer places a 2. May have to import random function.
}

fn is_valid_move(board: [[u32; 3]; 3], x: u32, y: u32) -> bool {
    // True if move is valid, else false.
    if x > 2 || y > 2 {
        return false;
    }
    else if board[x][y] != 0 {
        return false;
    }
    else {
        return true;
    }
}

fn main() {

    let mut board = [[0; 3]; 3];
    print_board(board);

    while keep_playing(board) {  // While the board isn't full and neither player has won, keep taking turns.
        player_move(board);  // Player is X, X goes first.
        print_board(board);
        if !keep_playing(board) {
            break;
        }
        computer_move(board); // Computer is O, O goes second.
        print_board(board);
    }

    let winner = get_winner(board);  // Print results.
    if winner==0{
        println!("Draw.");
    }
    else if winner==1{
        println!("You won!");
    }
    else {
        println!("You lost...");
    }

}
