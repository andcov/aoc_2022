use itertools::iproduct;
use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash)]
enum Dir {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmmds = read();
    let mut poss: HashSet<(i32, i32)> = HashSet::new();
    //               (x, y)
    let mut knots = [(0, 0); 10];

    poss.insert(knots[9]);

    for cmd in cmmds.iter() {
        match cmd {
            Dir::Right(val) => {
                for _ in 0..*val {
                    knots[0].0 += 1;
                    for i in 0..9 {
                        let h = knots[i].clone();
                        move_tail(&mut knots[i + 1], &h);
                    }
                    poss.insert(knots[9]);
                }
            }
            Dir::Left(val) => {
                for _ in 0..*val {
                    knots[0].0 -= 1;
                    for i in 0..9 {
                        let h = knots[i].clone();
                        move_tail(&mut knots[i + 1], &h);
                    }
                    poss.insert(knots[9]);
                }
            }
            Dir::Up(val) => {
                for _ in 0..*val {
                    knots[0].1 += 1;
                    for i in 0..9 {
                        let h = knots[i].clone();
                        move_tail(&mut knots[i + 1], &h);
                    }
                    poss.insert(knots[9]);
                }
            }
            Dir::Down(val) => {
                for _ in 0..*val {
                    knots[0].1 -= 1;
                    for i in 0..9 {
                        let h = knots[i].clone();
                        move_tail(&mut knots[i + 1], &h);
                    }
                    poss.insert(knots[9]);
                }
            }
        }
    }

    println!("The answer is: {}", poss.len());

    Ok(())
}

fn move_tail(t: &mut (i32, i32), h: &(i32, i32)) {
    for (dx, dy) in iproduct!((-1..=1), (-1..=1)) {
        if h.0 + dx == t.0 && h.1 + dy == t.1 {
            return;
        }
    }
    if t.0 < h.0 {
        t.0 += 1;
    }
    if t.0 > h.0 {
        t.0 -= 1;
    }
    if t.1 < h.1 {
        t.1 += 1;
    }
    if t.1 > h.1 {
        t.1 -= 1;
    }
}

fn read() -> Vec<Dir> {
    let input = std::fs::read_to_string("input/day09_1.in").unwrap();
    let mut res = vec![];
    let re = Regex::new(r"(\w) (\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let cmd = &caps[1];
        let amnt = caps[2].parse().unwrap();
        match cmd {
            "R" => res.push(Dir::Right(amnt)),
            "L" => res.push(Dir::Left(amnt)),
            "U" => res.push(Dir::Up(amnt)),
            "D" => res.push(Dir::Down(amnt)),
            _ => panic!("Unknown command"),
        }
    }

    res
}
