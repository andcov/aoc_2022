use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/day03_2.in")?;

    let mut priority = 0;
    let mut group_items: [[bool; 58]; 3] = [[false; 58]; 3];
    let mut cnt = 0;
    for line in input.lines() {
        for ch in line.chars() {
            group_items[cnt][((ch as u8) - 65) as usize] = true;
        }

        if cnt == 2 {
            for i in 0..58 {
                if group_items[0][i] && group_items[1][i] && group_items[2][i] {
                    if i < 26 {
                        priority += 27 + i;
                    } else {
                        priority += i - 32 + 1;
                    }
                    break;
                }
            }
            cnt = 0;
            group_items = [[false; 58]; 3];
        } else {
            cnt += 1;
        }
    }

    println!("The sum of the priorities is: {}", priority);

    Ok(())
}
