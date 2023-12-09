use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut ans: i64 = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut nums: Vec<i64> = Vec::new();
                for num_str in line.split(" ") {
                    nums.push(num_str.parse().unwrap());
                }

                let mut memo: Vec<Vec<i64>> = Vec::new();
                memo.push(nums);

                loop {
                    let mut new_diffs: Vec<i64> = Vec::new();
                    let diffs = memo.last().unwrap();
                    for i in 0..diffs.len() - 1 {
                        new_diffs.push(diffs[i + 1] - diffs[i]);
                    }

                    memo.push(new_diffs);
                    if memo.last().unwrap().iter().all(|&i| i == memo.last().unwrap()[0]) {
                        break;
                    }
                }

                memo.reverse();
                let mut now: i64 = 0;
                for diffs in &memo {
                    now += diffs.last().unwrap();
                }
                ans += now;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
    println!("{}", ans);
}
