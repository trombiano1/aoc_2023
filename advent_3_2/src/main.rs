use std::io::{self, BufRead};
use std::collections::HashMap;

fn check(i: i32, j: i32, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let n = map.len().try_into().unwrap();
    let m = map[0].len().try_into().unwrap();
    let mut count = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if i + di < 0 || i + di >= n {
                continue;
            }
            if j + dj < 0 || j + dj >= m {
                continue;
            }
            let ni: usize = (i + di).try_into().unwrap();
            let nj: usize = (j + dj).try_into().unwrap();
            if map[ni][nj] == '*' {
                count.push((ni, nj));
            }
        }
    }
    return count;
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();

    let stdin = io::stdin();
    let reader = stdin.lock();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut char_vec: Vec<char> = line.chars().collect();
                char_vec.push('.');
                map.push(char_vec);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
    
    let mut record: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 0]; map[0].len()]; map.len()];
    for (i, line) in map.iter().enumerate() {
        let mut now: i32 = 0;
        let mut gears = HashMap::new();

        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i32;
                now *= 10;
                now += digit;
                for (gi, gj) in check(i.try_into().unwrap(), j.try_into().unwrap(), &map) {
                    gears.insert((gi, gj), true);
                }
            } else {
                for ((gi, gj), _) in gears {
                    record[gi][gj].push(now);
                }
                now = 0;
                gears = HashMap::new();
            }
        }
    }

    let mut ans: i32 = 0;
    for line in record {
        for coord in line {
            if coord.len() == 2 {
                ans += coord[0] * coord[1];
            }
        }
    }
    println!("{}", ans);
}
