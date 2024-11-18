use std::io::{self, Write};

use board::{Board, Move};
use coordinates::Coordinates;

mod board;
mod coordinates;

fn main() {
    let mut board = Board::new();

    // Main game loop
    loop {
        board.print();

        // Loop until Player 1 makes a valid move
        loop {
            let input = prompt("Player 1 Move");

            let result = apply_move(&mut board, &input, Move::X);

            if result.is_err() {
                println!("Error: {}", result.err().unwrap());
                continue;
            }
            else {
                break;
            }
        }

        board.print();

        // Loop until Player 2 makes a valid move
        loop {
            let input = prompt("Player 2 Move");

            let result = apply_move(&mut board, &input, Move::O);

            if result.is_err() {
                println!("Error: {}", result.err().unwrap());
                continue;
            }
            else {
                break;
            }
        }
    }
}

fn apply_move<'a>(b: &'a mut Board, input: &'a String, player_move: Move) -> Result<&'a mut Board, &'a str> {
    let coordinate = Coordinates::from(input.trim())?;

    let move_result = b.apply(coordinate, player_move)?;

    return Ok(move_result);
}

fn prompt(msg: &str) -> String {
    let mut result = String::new();

    print!("{}> ", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut result).unwrap();

    result
}
