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

        if board.stalemate() {
            println!("Stalemate! Nobody wins!");
            std::process::exit(0);
        }

        // Loop until Player 1 makes a valid move
        loop {
            let input = prompt("Player 1 Move");

            let result = apply_move(&mut board, &input, Move::X);

            if result.is_err() {
                println!("Error: {}", result.err().unwrap());
                continue;
            } else {
                let win = board.check_win(&result.unwrap());

                if win.is_some() {
                    println!("Player 1 wins! {:#?}", win.unwrap());
                    std::process::exit(0);
                }

                break;
            }
        }

        board.print();

        if board.stalemate() {
            println!("Stalemate! Nobody wins!");
            std::process::exit(0);
        }

        // Loop until Player 2 makes a valid move
        loop {
            let input = prompt("Player 2 Move");

            let result = apply_move(&mut board, &input, Move::O);

            if result.is_err() {
                println!("Error: {}", result.err().unwrap());
                continue;
            } else {
                let win = board.check_win(&result.unwrap());

                if win.is_some() {
                    println!("Player 2 wins! {:#?}", win.unwrap());
                    std::process::exit(0);
                }

                break;
            }
        }
    }
}

fn apply_move(b: &mut Board, input: &String, player_move: Move) -> Result<Coordinates, String> {
    let coordinate = Coordinates::from(input.trim())?;

    let _move_result = b.apply(&coordinate, player_move)?;

    return Ok(coordinate);
}

fn prompt(msg: &str) -> String {
    let mut result = String::new();

    print!("{}> ", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut result).unwrap();

    result
}
