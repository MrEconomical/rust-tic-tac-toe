// Imports

use std::io;

// Tic tac toe board square

#[derive(PartialEq)]
enum Square {
    X,
    O,
    Empty
}

impl Square {
    // Check if square is empty

    fn is_empty(&self) -> bool {
        match self {
            Square::Empty => true,
            _ => false
        }
    }

    // Get game result from square type

    fn get_game_result(&self) -> Option<Result> {
        match self {
            Square::X => Some(Result::X),
            Square::O => Some(Result::O),
            Square::Empty => None
        }
    }

    // Get square character

    fn get_char(&self) -> char {
        match self {
            Square::X => 'X',
            Square::O => 'O',
            Square::Empty => '.'
        }
    }
}

// Player turn

enum Turn {
    X,
    O
}

impl Turn {
    // Get move square

    fn get_square(&self) -> Square {
        match self {
            Turn::X => Square::X,
            Turn::O => Square::O
        }
    }

    // Get turn of other player

    fn get_opposite(&self) -> Turn {
        match self {
            Turn::X => Turn::O,
            Turn::O => Turn::X
        }
    }

    // Get move character

    fn get_char(&self) -> char {
        match self {
            Turn::X => 'X',
            Turn::O => 'O'
        }
    }
}


// Game result of win or draw

enum Result {
    X,
    O,
    Draw
}

// Run Tic Tac Toe game

fn main() {
    // Initialize 3x3 board of empty squares and turn counter

    let mut board = vec![
        [Square::Empty, Square::Empty, Square::Empty],
        [Square::Empty, Square::Empty, Square::Empty],
        [Square::Empty, Square::Empty, Square::Empty]
    ];
    let mut turn = Turn::X;

    clear_screen();
    println!("Welcome to Bad Tic Tac Toe!");
    display_board(&board);

    // Start game loop

    loop {
        println!("Player {symbol}'s turn (place an {symbol})", symbol = turn.get_char());
        println!("Enter a square to make a move:");

        // Parse and validate player input

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get user input");
        let input = input.trim();
        let square = input.as_bytes();
        if square.len() != 2 {
            input_error(&input, &board);
            continue;
        }

        let square = (square[0] as char, square[1] as char);
        if (square.0 != 'a' && square.0 != 'b' && square.0 != 'c') ||
           (square.1 != '1' && square.1 != '2' && square.1 != '3') {
            input_error(&input, &board);
            continue;
        }

        let square: (usize, usize) = (
            if square.0 == 'a' { 0 } else if square.0 == 'b' { 1 } else { 2 },
            (square.1.to_digit(10).unwrap() - 1) as usize
        );

        // Make player move on board

        if board[square.0][square.1] == Square::Empty {
            board[square.0][square.1] = turn.get_square();
        } else {
            input_error(&input, &board);
            continue;
        }

        // Check for game result

        let result = check_result(&board);
        if let Some(result) = result {
            // End game on result

            clear_screen();
            match result {
                Result::X => println!("Player X wins!"),
                Result::O => println!("Player O wins!"),
                Result::Draw => println!("Game drawn!")
            }
            display_board(&board);
            break;
        }

        // End turn

        turn = turn.get_opposite();
        clear_screen();
        display_board(&board);
    }
}

// Clear terminal screen

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// Print board to terminal with labeled rows and columns

fn display_board(board: &[[Square; 3]]) {
    println!("  1 2 3");
    for r in 0..3 {
        let row_label = if r == 0 { "a" } else if r == 1 { "b" } else { "c" };
        print!("{row_label} ");
        for square in &board[r] {
            print!("{} ", square.get_char());
        }
        println!("{row_label}");
    }
    println!("  1 2 3");
}

// Display input error message

fn input_error(input: &str, board: &[[Square; 3]]) {
    clear_screen();
    println!("\"{input}\" is not a valid square, please try again (i.e. a1)");
    display_board(board);
}

// Check for player win or draw

fn check_result(board: &[[Square; 3]]) -> Option<Result> {
    // Check rows and columns

    for i in 0..3 {
        if !board[i][0].is_empty() && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return board[i][0].get_game_result();
        }
        if !board[0][i].is_empty() && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return board[i][0].get_game_result();
        }
    }

    // Check diagonals

    if !board[1][1].is_empty() && (
        (board[0][0] == board[1][1] && board[1][1] == board[2][2]) ||
        (board[0][2] == board[1][1] && board[1][1] == board[2][0])
    ) {
        return board[1][1].get_game_result();
    }

    // Check for draw

    for row in board {
        for square in row {
            if square.is_empty() {
                return None;
            }
        }
    }

    // All squares full

    Some(Result::Draw)
}