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
            state: [[Move::EMPTY; 3]; 3]
        }
    }

    pub fn apply(&mut self, coordinate: Coordinates, player_move: Move) -> Result<&mut Self, &str> {
        if coordinate.y as usize >= self.state.len() || coordinate.x as usize >= self.state[0].len() {
            return Err("Coordinates out of bounds");
        }

        // Check if space is occupied
        if self.state[coordinate.y as usize][coordinate.x as usize] != Move::EMPTY {
            return Err("Space is already occupied");
        }

        // Assign move
        self.state[coordinate.y as usize][coordinate.x as usize] = player_move;
        Ok(self)
    }
}
