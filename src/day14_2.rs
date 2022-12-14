use regex::Regex;

use self::Ele::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ele {
    Air,
    Rock,
    Sand,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut map, maxy) = read()?;

    let mut abyss = false;
    let mut cnt = 0;

    while !abyss {
        let mut pos = (500, 0);
        let mut flowing = true;
        while flowing {
            pos = if map[pos.0][pos.1 + 1] == Air {
                (pos.0, pos.1 + 1)
            } else if map[pos.0 - 1][pos.1 + 1] == Air {
                (pos.0 - 1, pos.1 + 1)
            } else if map[pos.0 + 1][pos.1 + 1] == Air {
                (pos.0 + 1, pos.1 + 1)
            } else {
                flowing = false;
                pos
            };
            if pos.1 == maxy + 1 {
                flowing = false;
            }
        }
        map[pos.0][pos.1] = Sand;
        cnt += 1;
        if pos == (500, 0) {
            abyss = true;
        }
    }
    println!("The asnwer is: {}", cnt);

    Ok(())
}

fn read() -> Result<([[Ele; 1000]; 1000], usize), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day14_2.in")?;
    let mut map = [[Air; 1000]; 1000];
    let mut maxy = usize::MIN;

    let re = Regex::new(r"(\d+),(\d+)")?;

    for l in input.lines() {
        for (curr, next) in re.captures_iter(l).zip(re.captures_iter(l).skip(1)) {
            let (x1, x2, y1, y2) = (
                curr[1].parse::<usize>()?,
                next[1].parse::<usize>()?,
                curr[2].parse::<usize>()?,
                next[2].parse::<usize>()?,
            );

            maxy = maxy.max(y1.max(y2));

            if x1 == x2 {
                (y1.min(y2)..=y1.max(y2)).for_each(|i| map[x1][i] = Rock);
            } else {
                (x1.min(x2)..=x1.max(x2)).for_each(|i| map[i][y1] = Rock);
            }
        }
    }

    Ok((map, maxy))
}
