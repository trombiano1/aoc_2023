use std::io::{self, BufRead};

fn dfs(i: isize, j: isize, k: usize, seen: &mut Vec<Vec<Vec<bool>>>, map: &Vec<Vec<char>>) {
    if i < 0 || i >= map.len() as isize || j < 0 || j >= map[0].len() as isize {
        return;
    }
    if seen[i as usize][j as usize][k] {
        return;
    }
    seen[i as usize][j as usize][k] = true;
    match (map[i as usize][j as usize], k) {
        ('|', 0 | 2) => {
            dfs(i - 1, j, 3, seen, map);
            dfs(i + 1, j, 1, seen, map);
        }
        ('-', 1 | 3) => {
            dfs(i, j + 1, 0, seen, map);
            dfs(i, j - 1, 2, seen, map);
        }
        ('/', _) => match k {
            0 => dfs(i - 1, j, 3, seen, map),
            1 => dfs(i, j - 1, 2, seen, map),
            2 => dfs(i + 1, j, 1, seen, map),
            3 => dfs(i, j + 1, 0, seen, map),
            _ => {}
        },
        ('\\', _) => match k {
            0 => dfs(i + 1, j, 1, seen, map),
            1 => dfs(i, j + 1, 0, seen, map),
            2 => dfs(i - 1, j, 3, seen, map),
            3 => dfs(i, j - 1, 2, seen, map),
            _ => {}
        },
        (_, _) => match k {
            0 => dfs(i, j + 1, k, seen, map),
            1 => dfs(i + 1, j, k, seen, map),
            2 => dfs(i, j - 1, k, seen, map),
            3 => dfs(i - 1, j, k, seen, map),
            _ => {}
        },
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
    dfs(0, 0, 0, &mut seen, &map);

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
