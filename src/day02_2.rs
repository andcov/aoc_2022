use core::fmt;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs;

#[derive(Default, Debug)]
struct MoveError(String);

impl Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an unknown code was used for a move")
    }
}

impl Error for MoveError {
    fn description(&self) -> &str {
        &self.0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day02_2.in")?;
    let mut score: u32 = 0;
    for line in input.lines() {
        let mut chars_iter = line.chars();
        let opp: char = chars_iter.next().expect("a move should have been here");
        chars_iter.next();
        let you: char = chars_iter.next().expect("a move should have been here");

        match you {
            'X' => {
                score += if opp == 'A' {
                    3
                } else if opp == 'B' {
                    1
                } else if opp == 'C' {
                    2
                } else {
                    0
                };
            }
            'Y' => {
                score += 3;
                score += if opp == 'A' {
                    1
                } else if opp == 'B' {
                    2
                } else if opp == 'C' {
                    3
                } else {
                    0
                };
            }
            'Z' => {
                score += 6;
                score += if opp == 'A' {
                    2
                } else if opp == 'B' {
                    3
                } else if opp == 'C' {
                    1
                } else {
                    0
                };
            }
            _ => return Err(Box::new(MoveError::default())),
        }
    }

    println!("The score is: {}", score);
    Ok(())
}
