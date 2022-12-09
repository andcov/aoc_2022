use itertools::iproduct;
use std::usize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, forest) = read()?;
    let mut max_res = 0;
    for (i, j) in iproduct!((1..(n - 1)), (1..(n - 1))) {
        let mut res = 1;
        let mut dir = 1;
        while i + dir < n && forest[i + dir][j] < forest[i][j] {
            dir += 1;
        }
        if i + dir == n {
            dir -= 1;
        }
        res *= dir;

        let mut dir = 1;
        while i >= dir && forest[i - dir][j] < forest[i][j] {
            dir += 1;
        }
        if i < dir {
            dir -= 1;
        }
        res *= dir;

        let mut dir = 1;
        while j + dir < n && forest[i][j + dir] < forest[i][j] {
            dir += 1;
        }
        if j + dir == n {
            dir -= 1;
        }
        res *= dir;

        let mut dir = 1;
        while j >= dir && forest[i][j - dir] < forest[i][j] {
            dir += 1;
        }
        if j < dir {
            dir -= 1;
        }
        res *= dir;

        max_res = res.max(max_res);
    }

    println!("The answer is: {}", max_res);

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
