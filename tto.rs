//tto.rs
use std::io;

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

fn player_move(board: &mut [[u32; 3]; 3]) {
    // Ask player for location on grid, then change to 1 (if not already taken).
    let mut invalid_pos: bool = true;
    while invalid_pos {
        println!("Enter row (0-2): ");
        let row: usize = read_input(); // Read user input for row
        println!("Enter column (0-2): ");
        let col: usize = read_input(); // Read user input for column
        
        if row < 3 && col < 3 {
            if board[row][col] == 0 {
                board[row][col] = 1;
                invalid_pos = false;
            } else {
                println!("Invalid move. Cell already taken. Please try another move.");
            }
        } else {
            println!("Move out of bounds. Please try another move.")
        }
        
    }
}

fn computer_move(board: [[u32; 3]; 3]) {
     // Computer places a 2. May have to import random function.
}

fn read_input() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn main() {

    let mut board = [[0; 3]; 3];
    print_board(board);

    while keep_playing(board) {  // While the board isn't full and neither player has won, keep taking turns.
        player_move(&mut board);  // Player is X, X goes first.
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
