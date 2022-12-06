use regex::Regex;
use std::fs;

/// value, previous index, next index
type Element = (char, usize, usize);

/// how many, from where, to where
type Instruction = (usize, usize, usize);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/day05_1.in")?;

    let mut dll: Vec<Element> = Vec::new();
    let mut first_ele: Vec<usize> = Vec::new();
    let mut last_ele: Vec<usize> = Vec::new();

    let mut instructions: Vec<Instruction> = Vec::new();

    let f_line = input.lines().next().unwrap();
    let cols = (f_line.len() + 1) / 4;
    for i in 0..cols {
        let ind = dll.len();
        dll.push((char::from_digit(1 + i as u32, 10).unwrap(), ind, ind));
        first_ele.push(ind);
        last_ele.push(ind);
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    let mut reading_data = true;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        if reading_data && chars[1] == '1' {
            reading_data = false;
        }
        if reading_data {
            for col in 0..cols {
                if chars[4 * col] == '[' {
                    let ind = dll.len();
                    let el: Element = (chars[4 * col + 1], 0, 0);
                    dll.push(el);
                    if first_ele[col] == last_ele[col] {
                        last_ele[col] = ind;
                        dll[first_ele[col]].2 = ind;
                        dll[ind].1 = first_ele[col];
                        dll[ind].2 = ind;
                    } else {
                        let sec_ind = dll[first_ele[col]].2;
                        dll[ind].1 = first_ele[col];
                        dll[ind].2 = sec_ind;
                        dll[first_ele[col]].2 = ind;
                        dll[sec_ind].1 = ind;
                    }
                }
            }
        } else {
            let caps = re.captures(line);
            if let Some(caps) = caps {
                instructions.push((caps[1].parse()?, caps[2].parse()?, caps[3].parse()?));
            }
        }
    }

    for instr in instructions {
        let (quant, sour_col, dest_col) = (instr.0, instr.1 - 1, instr.2 - 1);

        let mut new_last_ind = last_ele[sour_col];
        (dll[new_last_ind].1, dll[new_last_ind].2) = (dll[new_last_ind].2, dll[new_last_ind].1);
        for _ in 0..(quant - 1) {
            new_last_ind = dll[new_last_ind].2;
            (dll[new_last_ind].1, dll[new_last_ind].2) = (dll[new_last_ind].2, dll[new_last_ind].1);
        }

        dll[last_ele[dest_col]].2 = last_ele[sour_col];
        dll[last_ele[sour_col]].1 = last_ele[dest_col];
        last_ele[sour_col] = dll[new_last_ind].2;
        dll[last_ele[sour_col]].2 = last_ele[sour_col];
        last_ele[dest_col] = new_last_ind;
        dll[last_ele[dest_col]].2 = last_ele[dest_col];
    }

    print!("The answer is: ");
    last_ele.iter().for_each(|ind| print!("{}", dll[*ind].0));

    Ok(())
}
