use itertools::iproduct;
use std::sync::{Arc, Mutex};
use std::thread;
use std::usize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (n, forest) = read()?;

    let mut handles = vec![];

    let left = Arc::new(Mutex::new([[0u32; 100]; 100]));
    let left_ref = Arc::clone(&left);
    let handle = thread::spawn(move || {
        let mut left = left_ref.lock().unwrap();
        for (i, j) in iproduct!((0..n), (0..n)) {
            if j == 0 {
                left[i][j] = forest[i][j];
                continue;
            }
            left[i][j] = left[i][j - 1].max(forest[i][j]);
        }
    });
    handles.push(handle);

    let right = Arc::new(Mutex::new([[0u32; 100]; 100]));
    let right_ref = Arc::clone(&right);
    let handle = thread::spawn(move || {
        let mut right = right_ref.lock().unwrap();
        for (i, j) in iproduct!((0..n), (0..n).rev()) {
            if j == n - 1 {
                right[i][j] = forest[i][j];
                continue;
            }
            right[i][j] = right[i][j + 1].max(forest[i][j]);
        }
    });
    handles.push(handle);

    let top = Arc::new(Mutex::new([[0u32; 100]; 100]));
    let top_ref = Arc::clone(&top);
    let handle = thread::spawn(move || {
        let mut top = top_ref.lock().unwrap();
        for (j, i) in iproduct!((0..n), (0..n)) {
            if i == 0 {
                top[i][j] = forest[i][j];
                continue;
            }
            top[i][j] = top[i - 1][j].max(forest[i][j]);
        }
    });
    handles.push(handle);

    let bottom = Arc::new(Mutex::new([[0u32; 100]; 100]));
    let bottom_ref = Arc::clone(&bottom);
    let handle = thread::spawn(move || {
        let mut bottom = bottom_ref.lock().unwrap();
        for (j, i) in iproduct!((0..n).rev(), (0..n).rev()) {
            if i == n - 1 {
                bottom[i][j] = forest[i][j];
                continue;
            }
            bottom[i][j] = bottom[i + 1][j].max(forest[i][j]);
        }
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    let left = left.lock().unwrap();
    let right = right.lock().unwrap();
    let top = top.lock().unwrap();
    let bottom = bottom.lock().unwrap();
    let mut cnt = 4 * (n - 1);
    for (i, j) in iproduct!(1..(n - 1), 1..(n - 1)) {
        let val = forest[i][j];
        if val == left[i][j] && val != left[i][j - 1]
            || val == right[i][j] && val != right[i][j + 1]
            || val == top[i][j] && val != top[i - 1][j]
            || val == bottom[i][j] && val != bottom[i + 1][j]
        {
            cnt += 1;
        }
    }
    println!("The answer is: {}", cnt);
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
