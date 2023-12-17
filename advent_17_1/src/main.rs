use std::io::{self, BufRead};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in reader.lines() {
        let dat = line.unwrap();

        let mut now = Vec::new();
        for c in dat.chars().collect::<Vec<_>>() {
            now.push(c.to_digit(10).unwrap() as usize);
        }
        map.push(now);
    }

    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let inf = usize::MAX;
    let mut dist = vec![vec![vec![vec![inf; 3]; 4]; map[0].len()]; map.len()];
    let mut pque: BinaryHeap<(Reverse<_>, usize, usize, usize, usize)> = BinaryHeap::new();

    for d in 0..4 {
        dist[0][0][d][0] = 0;
        pque.push((Reverse(0), 0, 0, d, 0));
    }

    while !pque.is_empty() {
        let (Reverse(d), i, j, k, l) = pque.pop().unwrap();
        if dist[i][j][k][l] < d {
            continue;
        }

        for dir in 0..4 {
            if i as isize + dirs[dir].0 < 0
                || (i as isize + dirs[dir].0) as usize >= map.len()
                || j as isize + dirs[dir].1 < 0
                || (j as isize + dirs[dir].1) as usize >= map[0].len()
            {
                continue;
            }

            let ni: usize = (i as isize + dirs[dir].0) as usize;
            let nj: usize = (j as isize + dirs[dir].1) as usize;

            if dir == k {
                if l < 2 && dist[ni][nj][k][l + 1] > d + map[ni][nj] {
                    dist[ni][nj][k][l + 1] = dist[i][j][k][l] + map[ni][nj];
                    pque.push((Reverse(dist[ni][nj][k][l + 1]), ni, nj, k, l + 1));
                }
            } else if (dir as i32 - k as i32).abs() == 2 {
                continue;
            } else {
                if dist[ni][nj][dir][0] > dist[i][j][k][l] + map[ni][nj] {
                    dist[ni][nj][dir][0] = dist[i][j][k][l] + map[ni][nj];
                    pque.push((Reverse(dist[ni][nj][dir][0]), ni, nj, dir, 0));
                }
            }
        }
    }

    let ans = dist[map.len() - 1][map[0].len() - 1]
        .iter()
        .flatten()
        .min()
        .unwrap();
    println!("{}", ans);
}
