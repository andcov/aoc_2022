use regex::Regex;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/day04_1.in")?;

    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)")?;

    let mut cnt = 0;
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let (l1, r1, l2, r2) = (
            caps[1].parse::<usize>()?,
            caps[2].parse::<usize>()?,
            caps[3].parse::<usize>()?,
            caps[4].parse::<usize>()?,
        );
        if (l1 <= l2 && r1 >= r2) || (l1 >= l2 && r1 <= r2) {
            cnt += 1;
        }
    }
    println!("The answer is: {}", cnt);
    Ok(())
}
