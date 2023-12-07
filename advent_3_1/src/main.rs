use std::io::{self, BufRead};

fn check(i: i32, j: i32, map: &Vec<Vec<char>>) -> bool {
    let n = map.len().try_into().unwrap();
    let m = map[0].len().try_into().unwrap();
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
            if !map[ni][nj].is_digit(10) && map[ni][nj] != '.' {
                return true;
            }
        }
    }
    return false;
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
    
    let mut ans: i64 = 0;
    for (i, line) in map.iter().enumerate() {
        let mut now: i64 = 0;
        let mut ok = false;
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                let digit: i64 = c.to_digit(10).unwrap().into();
                now *= 10;
                now += digit;
                ok |= check(i.try_into().unwrap(), j.try_into().unwrap(), &map);
            } else {
                if ok {
                    ans += now;
                }
                ok = false;
                now = 0;
            }
        }
    }
    println!("{}", ans);
}
