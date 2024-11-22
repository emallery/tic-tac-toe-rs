use std::io::{self, Write};

use board::{Board, Move};
use coordinates::Coordinates;

mod board;
mod coordinates;

fn main() {
    let mut board: Board = Default::default();

    // Main game loop
    loop {
        println!("{}", board);

        if board.stalemate() {
            println!("Stalemate! Nobody wins!");
            std::process::exit(0);
        }

        // Handle Player 1's move
        if player_move(&mut board, Move::X, "Player 1 Move").is_some() {
            println!("{}", board);
            std::process::exit(0);
        };

        println!("{}", board);

        if board.stalemate() {
            println!("Stalemate! Nobody wins!");
            std::process::exit(0);
        }

        // Handle Player 2's move
        if player_move(&mut board, Move::O, "Player 2 Move").is_some() {
            println!("{}", board);
            std::process::exit(0);
        };
    }
}

fn player_move(board: &mut Board, player: Move, prompt_msg: &str) -> Option<Move> {
    loop {
        let input = prompt(prompt_msg).ok()?;

        match apply_move(board, &input, player) {
            Ok(coordinate) => {
                if let Some(win) = board.check_win(&coordinate) {
                    println!("Player {} wins! {:?}", player, win);
                    return Some(player);
                }
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }

    None
}

fn apply_move(b: &mut Board, input: &String, player_move: Move) -> Result<Coordinates, String> {
    let coordinate = Coordinates::from(input)?;

    let _move_result = b.apply(&coordinate, player_move)?;

    return Ok(coordinate);
}

fn prompt(msg: &str) -> Result<String, std::io::Error> {
    let mut result = String::new();
    print!("{}> ", msg);
    io::stdout().flush()?;
    io::stdin().read_line(&mut result)?;

    Ok(result.trim().to_string())
}
