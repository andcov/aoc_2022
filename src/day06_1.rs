use std::collections::HashSet;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chars: Vec<char> = fs::read_to_string("input/day06_1.in")?.chars().collect();
    for i in 0..(chars.len() - 4) {
        let mut curr_chars = HashSet::new();
        for j in 0..4 {
            curr_chars.insert(chars[i + j]);
        }
        if curr_chars.len() == 4 {
            println!("The answer is: {}", i + 4);
            return Ok(());
        }
    }
    Ok(())
}
