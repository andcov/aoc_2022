use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sensors_beacons = read()?;
    let sensors_dist = sensors_beacons
        .iter()
        .map(|(s, b)| (*s, dist(s, b)))
        .collect::<Vec<((i64, i64), i64)>>();

    let (mut x, mut y) = (0, 0);
    while y <= 4000000 {
        let mut max_d = 1;
        let mut is_in_range_any = false;
        for (s, d) in sensors_dist.iter() {
            if is_in_range(s, *d, &(x, y)) {
                let curr_d = *d - (y - s.1).abs() + (s.0 - x).abs() + 1;
                max_d = curr_d.max(max_d);
                is_in_range_any = true;
            }
        }
        if !is_in_range_any {
            println!("The answer is: {}", x * 4000000 + y);
            return Ok(());
        }
        x += max_d;
        if x > 4000000 {
            x = 0;
            y += 1;
        }
    }

    Ok(())
}

fn is_in_range(s: &(i64, i64), d: i64, pos: &(i64, i64)) -> bool {
    dist(s, pos) <= d
}

fn dist(s: &(i64, i64), b: &(i64, i64)) -> i64 {
    (s.0 as i64 - b.0 as i64).abs() + (s.1 as i64 - b.1 as i64).abs()
}

fn read() -> Result<Vec<((i64, i64), (i64, i64))>, Box<dyn std::error::Error>> {
    let mut res = vec![];
    let input = std::fs::read_to_string("input/day15_2.in")?;
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    for l in input.lines() {
        let caps = re.captures(l).unwrap();
        res.push((
            (caps[1].parse()?, caps[2].parse()?),
            (caps[3].parse()?, caps[4].parse()?),
        ));
    }

    Ok(res)
}
