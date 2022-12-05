use core::cmp::max;
use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("input/day01_1.in")?;
    let reader = BufReader::new(input_file);
    let mut max_sum: u32 = 0;
    let mut curr_sum: u32 = 0;
    for line in reader.lines() {
        match line?.parse::<u32>() {
            Ok(num) => curr_sum += num,
            Err(_) => {
                max_sum = max(max_sum, curr_sum);
                curr_sum = 0;
            }
        }
    }

    // once more for the last elf
    max_sum = max(max_sum, curr_sum);

    println!("The answer is: {}", max_sum);

    Ok(())
}
