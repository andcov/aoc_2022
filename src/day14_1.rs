use itertools::iproduct;
use regex::Regex;

use self::Ele::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ele {
    Air,
    Rock,
    Sand,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut map, minx, maxx, _, maxy) = read()?;

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
            if pos.0 < minx - 1 || pos.0 > maxx + 1 || pos.1 > maxy + 1 {
                flowing = false;
                abyss = true;
            }
        }
        map[pos.0][pos.1] = Sand;
        cnt += 1;
    }
    cnt -= 1;
    print_map(&map, maxy, minx, maxx);
    println!("The answer is: {}", cnt);

    Ok(())
}

fn print_map(map: &[[Ele; 1000]; 1000], maxy: usize, minx: usize, maxx: usize) {
    for (y, x) in iproduct!((0..=maxy), (minx..=maxx)) {
        print!(
            "{}",
            if map[x][y] == Air {
                "."
            } else if map[x][y] == Rock {
                "#"
            } else {
                "o"
            }
        );
        if x == maxx {
            println!()
        }
    }
}

fn read() -> Result<([[Ele; 1000]; 1000], usize, usize, usize, usize), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day14_2.in")?;
    let mut map = [[Air; 1000]; 1000];
    let (mut minx, mut maxx, mut miny, mut maxy) = (usize::MAX, usize::MIN, usize::MAX, usize::MIN);

    let re = Regex::new(r"(\d+),(\d+)")?;

    for l in input.lines() {
        for (curr, next) in re.captures_iter(l).zip(re.captures_iter(l).skip(1)) {
            let (x1, x2, y1, y2) = (
                curr[1].parse::<usize>()?,
                next[1].parse::<usize>()?,
                curr[2].parse::<usize>()?,
                next[2].parse::<usize>()?,
            );

            minx = minx.min(x1.min(x2));
            maxx = maxx.max(x1.max(x2));
            miny = miny.min(y1.min(y2));
            maxy = maxy.max(y1.max(y2));

            if x1 == x2 {
                (y1.min(y2)..=y1.max(y2)).for_each(|i| map[x1][i] = Rock);
            } else {
                (x1.min(x2)..=x1.max(x2)).for_each(|i| map[i][y1] = Rock);
            }
        }
    }

    Ok((map, minx, maxx, miny, maxy))
}
