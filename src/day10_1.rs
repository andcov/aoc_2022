use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day10_2.in")?;

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

    let s = vals
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(pos, val)| pos as i32 * (*val))
        .sum::<i32>();
    println!("The answer is: {}", s);

    Ok(())
}
