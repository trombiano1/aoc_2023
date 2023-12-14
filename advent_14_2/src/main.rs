use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn test_cycles(cycles: u64, pattern: &mut Vec<Vec<char>>, exit: bool) {
    let mut memo: HashMap<Vec<Vec<char>>, i64> = HashMap::new();
    for k in 0..cycles * 4 {
        let mut blocked: Vec<i32> = vec![-1; pattern[0].len()];
        match k % 4 {
            0 => blocked = vec![-1; pattern[0].len()],
            1 => blocked = vec![-1; pattern.len()],
            2 => blocked = vec![pattern.len() as i32; pattern[0].len()],
            3 => blocked = vec![pattern[0].len() as i32; pattern.len()],
            _ => {}
        }

        let mut new_pattern: Vec<Vec<char>> = vec![vec!['.'; pattern[0].len()]; pattern.len()];
        let mut ans = 0;
        for itr in 0..pattern.len() {
            for jtr in 0..pattern[0].len() {
                let i;
                let j;
                if (k % 4) < 2 {
                    i = itr;
                    j = jtr;
                } else {
                    i = pattern.len() - 1 - itr;
                    j = pattern[0].len() - 1 - jtr;
                }

                match pattern[i][j] {
                    '#' => {
                        match k % 2 {
                            0 => blocked[j] = i as i32,
                            1 => blocked[i] = j as i32,
                            _ => {}
                        }
                        new_pattern[i][j] = '#';
                    }
                    'O' => match k % 4 {
                        0 => {
                            blocked[j] = blocked[j] + 1;
                            new_pattern[blocked[j] as usize][j] = 'O';
                            ans += pattern.len() as i32 - (blocked[j] as i32);
                        }
                        1 => {
                            blocked[i] = blocked[i] + 1;
                            new_pattern[i][blocked[i] as usize] = 'O';
                            ans += pattern.len() as i32 - i as i32;
                        }
                        2 => {
                            blocked[j] = blocked[j] - 1;
                            new_pattern[blocked[j] as usize][j] = 'O';
                            ans += pattern.len() as i32 - (blocked[j] as i32);
                        }
                        3 => {
                            blocked[i] = blocked[i] - 1;
                            new_pattern[i][blocked[i] as usize] = 'O';
                            ans += pattern.len() as i32 - i as i32;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        std::mem::swap(pattern, &mut &mut new_pattern);
        if exit && k % 4 == 0 && memo.contains_key(&pattern.clone()) {
            println!("{} {}", k, memo[&pattern.clone()]);
            break;
        }
        if !exit && k == cycles * 4 - 1 {
            println!("{}", ans);
        }
        memo.insert(pattern.clone(), k as i64);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut pattern: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        pattern.push(line.unwrap().chars().collect());
    }

    let cycles: u64 = 1000000000;
    test_cycles(cycles, &mut pattern.clone(), true);
    test_cycles(372 + 60, &mut pattern, false);
}
