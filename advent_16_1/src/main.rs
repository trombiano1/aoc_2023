use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

use Direction::*;

fn dfs(i: isize, j: isize, k: Direction, seen: &mut Vec<Vec<Vec<bool>>>, map: &Vec<Vec<char>>) {
    if i < 0 || i >= map.len() as isize || j < 0 || j >= map[0].len() as isize {
        return;
    }
    if seen[i as usize][j as usize][k as usize] {
        return;
    }
    seen[i as usize][j as usize][k as usize] = true;
    match (map[i as usize][j as usize], k) {
        ('|', Right | Left) => {
            dfs(i - 1, j, Up, seen, map);
            dfs(i + 1, j, Down, seen, map);
        }
        ('-', Down | Up) => {
            dfs(i, j + 1, Right, seen, map);
            dfs(i, j - 1, Left, seen, map);
        }
        ('/', _) => match k {
            Right => dfs(i - 1, j, Up, seen, map),
            Down => dfs(i, j - 1, Left, seen, map),
            Left => dfs(i + 1, j, Down, seen, map),
            Up => dfs(i, j + 1, Right, seen, map),
        },
        ('\\', _) => match k {
            Right => dfs(i + 1, j, Down, seen, map),
            Down => dfs(i, j + 1, Right, seen, map),
            Left => dfs(i - 1, j, Up, seen, map),
            Up => dfs(i, j - 1, Left, seen, map),
        },
        (_, Right) => dfs(i, j + 1, Right, seen, map),
        (_, Down) => dfs(i + 1, j, Down, seen, map),
        (_, Left) => dfs(i, j - 1, Left, seen, map),
        (_, Up) => dfs(i - 1, j, Up, seen, map),
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let dat = line.unwrap();
        map.push(dat.chars().collect());
    }

    let mut seen = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
    dfs(0, 0, Right, &mut seen, &map);

    let mut ans = 0;
    for l in &seen {
        for c in l {
            if c[0] || c[1] || c[2] || c[3] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
