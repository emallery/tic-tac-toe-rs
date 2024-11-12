use std::io::{self, Write};

use board::{Board, Move};
use coordinates::Coordinates;

mod board;
mod coordinates;

fn main() {
    println!("Hello, world!");

    let mut b = Board::new();

    // Main game loop
    loop {
        b.print();

        let aaa = loop {
            let input = prompt("Player 1 Move");

            match Coordinates::from(&input.trim()) {
                Ok(coordinates) => break coordinates,
                Err(e) => {
                    println!("Error: {}", e);
                    () // ???
                },
            }
        };

        println!("Move is at: {:?}", aaa);
        b.apply(aaa, Move::X);

        b.print();

        let aaa = loop {
            let input = prompt("Player 2 Move");

            match Coordinates::from(&input.trim()) {
                Ok(coordinates) => break coordinates,
                Err(e) => {
                    println!("Error: {}", e);
                    () // ???
                },
            }
        };

        println!("Move is at: {:?}", aaa);
        b.apply(aaa, Move::O);
    }
}

fn prompt(msg: &str) -> String {
    let mut result = String::new();

    print!("{}> ", msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut result).unwrap();

    result
}
