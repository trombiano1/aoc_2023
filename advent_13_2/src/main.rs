use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let dat = line.unwrap();
        if i == 0 || dat == "" {
            patterns.push(Vec::new());
        }
        if dat == "" {
            continue;
        }

        patterns
            .last_mut()
            .unwrap()
            .push(dat.chars().flat_map(|c| vec![c, ' ']).collect());
        patterns.last_mut().unwrap().push(Vec::new());
    }

    let mut ans = 0;
    for pattern in patterns {
        for r in (1..pattern.len() - 1).step_by(2) {
            let mut count = 0;
            'test: for i in (0..pattern.len()).step_by(2) {
                for j in (0..pattern[0].len()).step_by(2) {
                    if i <= 2 * r && 2 * r - i < pattern.len() {
                        if pattern[i][j] != pattern[2 * r - i][j] {
                            count += 1;
                        }
                    }
                }
            }
            if count == 2 {
                ans += 100 * ((r + 1) / 2);
                break;
            }
        }

        for c in (1..pattern[0].len() - 1).step_by(2) {
            let mut count = 0;
            'test: for i in (0..pattern.len()).step_by(2) {
                for j in (0..pattern[0].len()).step_by(2) {
                    if j <= 2 * c && 2 * c - j < pattern[0].len() {
                        if pattern[i][j] != pattern[i][2 * c - j] {
                            count += 1;
                        }
                    }
                }
            }
            if count == 2 {
                ans += (c + 1) / 2;
                break;
            }
        }
    }

    println!("{}", ans);
}

/*
01234567890123456
# . # # . . # # .

*/
