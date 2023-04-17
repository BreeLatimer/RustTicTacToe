use rand::Rng;

fn print_board(board: &[[u32; 3]; 3]) {
    // Prints the contents of the board. 1 is X, 2 is O, others blank.
    for row in *board {
        let mut output = String::from("");
        for cell in row {
            if cell == 0 {
                output.push_str("[ ]");
            } else if cell == 1 {
                output.push_str("[X]");
            } else {
                output.push_str("[O]");
            }
        }
        println!("{}", output);
    }
}

fn keep_playing(board: &[[u32; 3]; 3]) -> bool {
    // False if board full or either player's won, else true.

    if board_full(board) {
        return false;
    } else if get_winner(board) != 0 {
        return false;
    } else {
        return true;
    }
}

fn board_full(board: &[[u32; 3]; 3]) -> bool {
    // True if board full, else false.
    for row in *board {
        for cell in row {
            if cell == 0 {
                return false;
            }
        }
    }

    true // if we get to this point the board is full.
}

fn get_winner(board: &[[u32; 3]; 3]) -> i32 {
    // 0 if none, 1 if Player, 2 if computer.
    // Check rows.
    for row in board {
        if row[0] == row[1] && row[1] == row[2] {
            if row[0] == 1 {
                return 1;
            } else if row[0] == 2 {
                return 2;
            }
        }
    }
    // Check columns.
    for i in 0..3 {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            if board[0][i] == 1 {
                return 1;
            } else if board[0][i] == 2 {
                return 2;
            }
        }
    }
    // Check diagonals.
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        if board[0][0] == 1 {
            return 1;
        } else if board[0][0] == 2 {
            return 2;
        }
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        if board[0][2] == 1 {
            return 1;
        } else if board[0][2] == 2 {
            return 2;
        }
    }
    0 // if we get to this point, no winner yet.
}

fn player_move(board: &mut [[u32; 3]; 3]) {
    // Ask player for location on grid, then change to 1 (if not already taken).
    //Moves are provided as two letters: rows can be (l,m,r) and columns can be (t,m,b).
    //These can be provided in any order, so (l,t) is the same as (t,l).
    let mut has_valid_move = false;
    let mut x = 0;
    let mut y = 0;

    while !has_valid_move {
        // Get input.
        let mut input = String::new();
        println!("Enter your move: ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // TODO: check length of input and if the chars are valid.

        // Convert input to coordinates.
        let mut input = input.chars();
        x = 0;
        y = 0;
        let mut x_set = false;
        let mut y_set = false;
        while let Some(c) = input.next() {
            if c == 'l' && !x_set {
                x = 0;
                x_set = true;
            } else if c == 'r' && !x_set {
                x = 2;
                x_set = true;
            } else if c == 't' && !y_set {
                y = 0;
                y_set = true;
            } else if c == 'b' && !y_set {
                y = 2;
                y_set = true;
            } else if c == 'm' && !y_set {
                y = 1;
                y_set = true;
            } else if c == 'm' && !x_set {
                x = 1;
                x_set = true;
            }
        }
        if is_valid_move(board, x, y) {
            has_valid_move = true;
        } else {
            println!("Invalid move. Try again.");
        }
    }
    board[y][x] = 1;
}

fn computer_move(board: &mut [[u32; 3]; 3]) {
    // if first
    if is_valid_move(board, 1, 1) {
        board[1][1] = 2;
        return;
    }

    // Check for winning move/move to block.
    for i in 0..3 {
        for j in 0..3 {
            if is_valid_move(board, i, j) {
                board[j][i] = 2;
                if get_winner(board) == 2 {
                    return;
                } else {
                    board[j][i] = 1;
                    if get_winner(board) == 1 {
                        board[j][i] = 2;
                        return;
                    } else {
                        board[j][i] = 0;
                    }
                }
            }
        }
    }

    // do random valid move.
    let mut valid_moves: Vec<(usize, usize)> = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            if is_valid_move(board, i, j) {
                valid_moves.push((i, j));
            }
        }
    }
    let mut rng = rand::thread_rng();
    let choice = rng.gen_range(0..valid_moves.len());
    board[valid_moves[choice].1][valid_moves[choice].0] = 2;
}

fn is_valid_move(board: &[[u32; 3]; 3], x: usize, y: usize) -> bool {
    // True if move is valid, else false.
    if x > 2 || y > 2 {
        return false;
    } else if board[y][x] != 0 {
        return false;
    } else {
        return true;
    }
}

fn main() {
    let mut board = [[0; 3]; 3];
    println!("Welcome to Tic-Tac-Toe! \n You are X, the computer is O. \n Moves are provided as two letters: rows can be (l,m,r) and columns can be (t,m,b). \n These can be provided in any order, so `lt` is the same as `tl`. \n Good luck! \n");
    while keep_playing(&board) {
        // While the board isn't full and neither player has won, keep taking turns.
        player_move(&mut board); // Player is X, X goes first.
        if !keep_playing(&board) {
            break;
        }
        computer_move(&mut board); // Computer is O, O goes second.
        print_board(&board);
    }

    let winner = get_winner(&board); // Print results.
    if winner == 0 {
        println!("Draw.");
    } else if winner == 1 {
        println!("You won!");
    } else {
        println!("You lost...");
    }
}
