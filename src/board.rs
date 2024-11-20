use crate::coordinates::Coordinates;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    EMPTY,
    X,
    O,
}

impl Move {
    fn as_str(&self) -> &'static str {
        match self {
            Move::EMPTY => " ",
            Move::X => "X",
            Move::O => "O",
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub struct Board {
    state: [[Move; 3]; 3],
    moves_remaining: i32,
}

impl Board {
    pub fn print(&self) {
        println!(
            "  A   B   C
1 {} | {} | {}
 -----------
2 {} | {} | {}
 -----------
3 {} | {} | {}",
            self.state[0][0],
            self.state[0][1],
            self.state[0][2],
            self.state[1][0],
            self.state[1][1],
            self.state[1][2],
            self.state[2][0],
            self.state[2][1],
            self.state[2][2]
        )
    }

    pub fn new() -> Board {
        Board {
            state: [[Move::EMPTY; 3]; 3],
            moves_remaining: 9,
        }
    }

    pub fn apply(
        &mut self,
        coordinate: &Coordinates,
        player_move: Move,
    ) -> Result<&mut Self, &str> {
        if coordinate.y as usize >= self.state.len() || coordinate.x as usize >= self.state[0].len()
        {
            return Err("Coordinates out of bounds");
        }

        // Check if space is occupied
        if self.state[coordinate.y as usize][coordinate.x as usize] != Move::EMPTY {
            return Err("Space is already occupied");
        }

        // Assign move
        self.state[coordinate.y as usize][coordinate.x as usize] = player_move;
        self.moves_remaining -= 1;
        Ok(self)
    }

    pub fn check_win(&mut self, last_move: &Coordinates) -> Option<[Coordinates; 3]> {
        let letter = self.state[last_move.y as usize][last_move.x as usize];

        // Check the row the last move was played in
        for x in 0..3 {
            if self.state[last_move.y as usize][x] != letter {
                break;
            }

            if x == 2 {
                return Some([
                    Coordinates {
                        x: 0,
                        y: last_move.y,
                    },
                    Coordinates {
                        x: 1,
                        y: last_move.y,
                    },
                    Coordinates {
                        x: 2,
                        y: last_move.y,
                    },
                ]);
            }
        }

        // Check column
        for y in 0..3 {
            if self.state[y][last_move.x as usize] != letter {
                break;
            }

            if y == 2 {
                return Some([
                    Coordinates {
                        x: last_move.x,
                        y: 0,
                    },
                    Coordinates {
                        x: last_move.x,
                        y: 1,
                    },
                    Coordinates {
                        x: last_move.x,
                        y: 2,
                    },
                ]);
            }
        }

        // Check diagonal
        if last_move.x == last_move.y {
            for i in 0..3 {
                if self.state[i][i] != letter {
                    break;
                }

                if i == 2 {
                    return Some([
                        Coordinates { x: 0, y: 0 },
                        Coordinates { x: 1, y: 1 },
                        Coordinates { x: 2, y: 2 },
                    ]);
                }
            }
        }

        if last_move.x + last_move.y == 2 {
            for i in 0..3 {
                if self.state[i][2 - i] != letter {
                    break;
                }

                if i == 2 {
                    return Some([
                        Coordinates { x: 0, y: 2 },
                        Coordinates { x: 1, y: 1 },
                        Coordinates { x: 2, y: 0 },
                    ]);
                }
            }
        }

        None
    }

    pub fn stalemate(&self) -> bool {
        self.moves_remaining <= 0
    }
}
