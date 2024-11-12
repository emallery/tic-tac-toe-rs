
#[derive(Debug)]
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
                if x.is_some() {
                    return Err("Invalid coordinate format: can't re-map x-coordinate");
                }
                else {
                    x = Some(i.to_ascii_uppercase() as i32 - 'A'.to_ascii_uppercase() as i32)
                }
            }

            // Parse a Y coordinate
            let digit = i.to_digit(10);
            if digit.is_some() {
                if y.is_some() {
                    return Err("Invalid coordinate format: can't re-map y-coordinate");
                }
                else {
                    y = Some(digit.unwrap() as i32 - 1);
                }
            }
        }

        // Make sure both coordinates were set
        if x.is_none() || y.is_none() {
            return Err("Missing X or Y coordinate")
        }

        Ok(Coordinates {x: x.unwrap(), y: y.unwrap()})
    }
}
