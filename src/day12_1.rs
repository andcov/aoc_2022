use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = [[0; 165]; 165];
    let mut n = 0;
    let mut m = 0;
    let (mut start_pos, mut end_pos) = ((0, 0), (0, 0));

    read(&mut n, &mut m, &mut map, &mut start_pos, &mut end_pos);

    let mut q = VecDeque::new();
    q.push_back(start_pos);

    let mut vis = [[false; 165]; 165];
    let mut val = [[0; 165]; 165];

    vis[start_pos.0][start_pos.1] = true;
    while let Some((i, j)) = q.pop_front() {
        for ne in get_neighbours(&(i, j), &(n, m), &map) {
            if !vis[ne.0][ne.1] {
                val[ne.0][ne.1] = val[i][j] + 1;
                q.push_back(ne);
                vis[ne.0][ne.1] = true;
            }
        }

        if val[end_pos.0][end_pos.1] != 0 {
            break;
        }
    }

    println!("The answer is: {}", val[end_pos.0][end_pos.1]);

    Ok(())
}

fn get_neighbours(
    pos: &(usize, usize),
    size: &(usize, usize),
    map: &[[u8; 165]; 165],
) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let di = vec![-1, 0, 1, 0].into_iter();
    let dj = vec![0, 1, 0, -1].into_iter();

    for (di, dj) in di.zip(dj) {
        if (pos.0 as i32 + di) >= 0
            && (pos.0 as i32 + di) < size.0 as i32
            && (pos.1 as i32 + dj) >= 0
            && (pos.1 as i32 + dj) < size.1 as i32
            && (map[(pos.0 as i32 + di) as usize][(pos.1 as i32 + dj) as usize]
                <= map[pos.0][pos.1]
                || map[(pos.0 as i32 + di) as usize][(pos.1 as i32 + dj) as usize]
                    == map[pos.0][pos.1] + 1)
        {
            res.push(((pos.0 as i32 + di) as usize, (pos.1 as i32 + dj) as usize));
        }
    }

    res
}

fn read(
    n: &mut usize,
    m: &mut usize,
    map: &mut [[u8; 165]; 165],
    start_pos: &mut (usize, usize),
    end_pos: &mut (usize, usize),
) {
    let input = std::fs::read_to_string("input/day12_2.in").unwrap();
    *n = 0 as usize;
    *m = 0 as usize;

    for (i, l) in input.lines().enumerate() {
        *n += 1;
        *m = l.len();
        for (j, c) in l.chars().enumerate() {
            match c {
                'S' => {
                    map[i][j] = 'a' as u8 - 97;
                    *start_pos = (i, j);
                }
                'E' => {
                    map[i][j] = 'z' as u8 - 97;
                    *end_pos = (i, j);
                }
                _ => map[i][j] = c as u8 - 97,
            };
        }
    }
}
