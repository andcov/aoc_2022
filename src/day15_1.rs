use std::collections::HashSet;

use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sensors_beacons = read()?;
    let sensors_dist = sensors_beacons
        .iter()
        .map(|(s, b)| (*s, dist(*s, *b)))
        .collect::<Vec<((i64, i64), i64)>>();

    let y_target = 2000000;
    let mut occ_vals =
        sensors_dist
            .iter()
            .fold(HashSet::new(), |mut acc, (s, d)| -> HashSet<i64> {
                if (s.1 - y_target).abs() <= *d {
                    let rem = d - (s.1 - y_target).abs();
                    ((s.0 - rem)..=(s.0 + rem)).for_each(|i| {
                        acc.insert(i);
                    });
                }
                acc
            });

    sensors_beacons.iter().for_each(|(_, b)| {
        if b.1 == y_target {
            occ_vals.remove(&b.1);
        }
    });

    println!("The answer is: {:?}", occ_vals.len());
    Ok(())
}

fn dist(s: (i64, i64), b: (i64, i64)) -> i64 {
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
