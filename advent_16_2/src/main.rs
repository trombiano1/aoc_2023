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

    let mut ans = 0;
    for i in 0..map.len() {
        let mut seen = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        dfs(i as isize, 0, Right, &mut seen, &map);
        let mut now = 0;
        for l in &seen {
            for c in l {
                if c[0] || c[1] || c[2] || c[3] {
                    now += 1;
                }
            }
        }
        ans = std::cmp::max(ans, now);
    }
    for j in 0..map[0].len() {
        let mut seen = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        dfs(0, j as isize, Down, &mut seen, &map);
        let mut now = 0;
        for l in &seen {
            for c in l {
                if c[0] || c[1] || c[2] || c[3] {
                    now += 1;
                }
            }
        }
        ans = std::cmp::max(ans, now);
    }
    for i in 0..map.len() {
        let mut seen = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        dfs(
            i as isize,
            (map[0].len() - 1) as isize,
            Left,
            &mut seen,
            &map,
        );
        let mut now = 0;
        for l in &seen {
            for c in l {
                if c[0] || c[1] || c[2] || c[3] {
                    now += 1;
                }
            }
        }
        ans = std::cmp::max(ans, now);
    }
    for j in 0..map[0].len() {
        let mut seen = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
        dfs((map.len() - 1) as isize, j as isize, Up, &mut seen, &map);
        let mut now = 0;
        for l in &seen {
            for c in l {
                if c[0] || c[1] || c[2] || c[3] {
                    now += 1;
                }
            }
        }
        ans = std::cmp::max(ans, now);
    }

    println!("{}", ans);
}
