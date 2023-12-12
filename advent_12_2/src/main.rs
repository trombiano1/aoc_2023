use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut ans = 0;
    for line in reader.lines() {
        let dat = line.unwrap();
        let mut parts = dat.split(" ");
        let folded_map = parts.next().unwrap().to_owned();
        let counts_str = parts.next().unwrap();
        let mut folded_counts: Vec<usize> = Vec::new();
        for count_str in counts_str.split(",") {
            folded_counts.push(count_str.parse().unwrap());
        }
        let max = folded_counts.iter().max().unwrap().clone();

        let map = vec![folded_map; 5].join("?");
        let counts: Vec<_> = std::iter::repeat(folded_counts).take(5).flatten().collect();

        let mut dp: Vec<Vec<usize>> = vec![vec![0; max + 1]; counts.len() + 1];
        dp[0][0] = 1;
        for c in map.chars() {
            let mut new_dp: Vec<Vec<usize>> = vec![vec![0; max + 1]; counts.len() + 1];
            for j in 0..=counts.len() {
                for k in 0..=max {
                    match c {
                        '.' => {
                            if k == 0 {
                                new_dp[j][k] += dp[j][k];
                            }
                            if j < counts.len() && k == counts[j] {
                                new_dp[j + 1][0] += dp[j][k];
                            }
                        }
                        '?' => {
                            if k == 0 {
                                new_dp[j][k] += dp[j][k];
                            }
                            if j < counts.len() && k < counts[j] {
                                new_dp[j][k + 1] += dp[j][k];
                            }
                            if j < counts.len() && k == counts[j] {
                                new_dp[j + 1][0] += dp[j][k];
                            }
                        }
                        '#' => {
                            if j < counts.len() && k < counts[j] {
                                new_dp[j][k + 1] += dp[j][k];
                            }
                        }
                        _ => {}
                    }
                }
            }
            std::mem::swap(&mut new_dp, &mut dp);
        }
        ans += dp[counts.len() - 1][counts.last().unwrap().clone()] + dp.last().unwrap()[0];
    }
    println!("{}", ans);
}
