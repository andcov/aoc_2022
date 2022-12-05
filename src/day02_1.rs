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
    let input = fs::read_to_string("input/day02_1.in")?;
    let mut score: u32 = 0;
    for line in input.lines() {
        let mut chars_iter = line.chars();
        let opp: char = chars_iter.next().expect("a move should have been here");
        chars_iter.next();
        let you: char = chars_iter.next().expect("a move should have been here");

        match you {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _ => return Err(Box::new(MoveError::default())),
        }

        // draw
        if (you == 'X' && opp == 'A') || (you == 'Y' && opp == 'B') || (you == 'Z' && opp == 'C') {
            score += 3;
        } else if (you == 'X' && opp == 'C')
            || (you == 'Y' && opp == 'A')
            || (you == 'Z' && opp == 'B')
        {
            score += 6;
        }
    }

    println!("The score is: {}", score);
    Ok(())
}
