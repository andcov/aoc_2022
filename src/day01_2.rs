use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("input/day01_2.in")?;
    let reader = BufReader::new(input_file);
    let mut max_sums: [u32; 3] = [0, 0, 0];
    let mut curr_sum: u32 = 0;
    for line in reader.lines() {
        match line?.parse::<u32>() {
            Ok(num) => curr_sum += num,
            Err(_) => {
                if curr_sum > max_sums[0] {
                    (max_sums[1], max_sums[2]) = (max_sums[2], max_sums[1]);
                    (max_sums[0], max_sums[1]) = (max_sums[1], max_sums[0]);
                    max_sums[0] = curr_sum;
                } else if curr_sum > max_sums[1] {
                    (max_sums[1], max_sums[2]) = (max_sums[2], max_sums[1]);
                    max_sums[1] = curr_sum;
                } else if curr_sum > max_sums[2] {
                    max_sums[2] = curr_sum;
                }
                curr_sum = 0;
            }
        }
    }
    // once more for the last elf
    if curr_sum > max_sums[0] {
        (max_sums[1], max_sums[2]) = (max_sums[2], max_sums[1]);
        (max_sums[0], max_sums[1]) = (max_sums[1], max_sums[0]);
        max_sums[0] = curr_sum;
    } else if curr_sum > max_sums[1] {
        (max_sums[1], max_sums[2]) = (max_sums[2], max_sums[1]);
        max_sums[1] = curr_sum;
    } else if curr_sum > max_sums[2] {
        max_sums[2] = curr_sum;
    }

    println!("The answer is: {}", max_sums[0] + max_sums[1] + max_sums[2]);

    Ok(())
}
