use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/day03_1.in")?;

    let mut priority = 0;
    for line in input.lines() {
        let mut item_in_first: [bool; 58] = [false; 58];
        let mut item_in_sec: [bool; 58] = [false; 58];
        for (i, ch) in line.chars().enumerate() {
            if i < line.len() / 2 {
                item_in_first[((ch as u8) - 65) as usize] = true;
            } else {
                item_in_sec[((ch as u8) - 65) as usize] = true;
            }
        }
        for i in 0..58 {
            if item_in_first[i] && item_in_sec[i] {
                if i < 26 {
                    priority += 27 + i;
                } else {
                    priority += i - 32 + 1;
                }
                break;
            }
        }
    }

    println!("The sum of the priorities is: {}", priority);

    Ok(())
}
