use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day10_1.in")?;

    let mut val = 1;
    let add_re = Regex::new(r"addx (-?\d+)")?;
    let mut vals = vec![];
    vals.push(0);
    vals.push(1);

    for line in input.lines() {
        if let Some(caps) = add_re.captures(line) {
            vals.push(val);
            val += caps[1].parse::<i32>()?;
            vals.push(val);
        } else {
            vals.push(val);
        }
    }

    for i in 1..(vals.len() - 1) {
        if (i as i32 - 1) % 40 == vals[i]
            || (i as i32 - 1) % 40 == vals[i] + 1
            || (i as i32 - 1) % 40 == vals[i] - 1
        {
            print!("##");
        } else {
            print!("  ");
        }
        if i % 40 == 0 {
            println!()
        }
    }

    Ok(())
}
