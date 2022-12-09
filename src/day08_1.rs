use std::usize;

use itertools::{iproduct, Itertools};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, forest) = read()?;
    let mut left = [[0u32; 100]; 100];
    let mut right = [[0u32; 100]; 100];
    let mut top = [[0u32; 100]; 100];
    let mut bottom = [[0u32; 100]; 100];

    for (i, j) in iproduct!((0..n), (0..n)) {
        if j == 0 {
            left[i][j] = forest[i][j];
            continue;
        }
        left[i][j] = left[i][j - 1].max(forest[i][j]);
    }

    for (i, j) in iproduct!((0..n), (0..n).rev()) {
        if j == n - 1 {
            right[i][j] = forest[i][j];
            continue;
        }
        right[i][j] = right[i][j + 1].max(forest[i][j]);
    }

    for (j, i) in iproduct!((0..n), (0..n)) {
        if i == 0 {
            top[i][j] = forest[i][j];
            continue;
        }
        top[i][j] = top[i - 1][j].max(forest[i][j]);
    }

    for (j, i) in iproduct!((0..n).rev(), (0..n).rev()) {
        if i == n - 1 {
            bottom[i][j] = forest[i][j];
            continue;
        }
        bottom[i][j] = bottom[i + 1][j].max(forest[i][j]);
    }
    Ok(())
}

fn read() -> Result<(usize, [[u32; 100]; 100]), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/day08_2.in")?;
    let mut res = [[0; 100]; 100];
    let mut n = 0;

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            res[i][j] = c.to_digit(10).unwrap();
        }
        n += 1;
    }

    Ok((n, res))
}

/*
*
   for (i, j) in iproduct!((0..n), (0..n)) {
       print!("{}", left[i][j]);
       if j == n - 1 {
           println!();
       }
   }
   println!();

   for (i, j) in iproduct!((0..n), (0..n)) {
       print!("{}", right[i][j]);
       if j == n - 1 {
           println!();
       }
   }
   println!();

   for (i, j) in iproduct!((0..n), (0..n)) {
       print!("{}", top[i][j]);
       if j == n - 1 {
           println!();
       }
   }
   println!();

   for (i, j) in iproduct!((0..n), (0..n)) {
       print!("{}", bottom[i][j]);
       if j == n - 1 {
           println!();
       }
   }
   println!();

* */
