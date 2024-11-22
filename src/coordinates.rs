#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    // Jank incoming
    pub fn from(str: &str) -> Result<Coordinates, &str> {
        if str.len() != 2 {
            return Err("Coordinate string must contain exactly two characters");
        }

        let mut x: Option<i32> = None;
        let mut y: Option<i32> = None;

        for i in str.chars() {
            // Parse an X coordinate
            if i.is_ascii_alphabetic() {
                x = Some(i.to_ascii_uppercase() as i32 - 'A' as i32);
            }
            // Parse a Y coordinate
            else if let Some(digit) = i.to_digit(10) {
                y = Some(digit as i32 - 1);
            }
        }

        // Make sure both coordinates were set
        if let (Some(x), Some(y)) = (x, y) {
            return Ok(Coordinates { x, y });
        } else {
            return Err("Missing X or Y coordinate");
        }
    }
}
