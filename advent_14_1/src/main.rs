use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut pattern: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        pattern.push(line.unwrap().chars().collect());
    }

    let mut blocked: Vec<i32> = vec![-1; pattern[0].len()];

    let mut ans = 0;
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            match pattern[i][j] {
                '#' => blocked[j] = i as i32,
                'O' => {
                    blocked[j] = blocked[j] + 1;
                    ans += pattern.len() as i32 - (blocked[j] as i32);
                }
                _ => {}
            }
        }
    }

    println!("{}", ans);
}
