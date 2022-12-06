use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chars: Vec<char> = fs::read_to_string("input/day06_2.in")?.chars().collect();
    for i in 0..(chars.len() - 14) {
        let mut curr_chars = HashSet::new();
        for j in 0..14 {
            curr_chars.insert(chars[i + j]);
        }
        if curr_chars.len() == 14 {
            println!("The answer is: {}", i + 14);
            return Ok(());
        }
    }
    Ok(())
}
