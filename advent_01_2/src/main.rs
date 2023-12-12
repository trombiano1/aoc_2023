use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let nums: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string(),
                               "four".to_string(), "five".to_string(), "six".to_string(),
                               "seven".to_string(), "eight".to_string(), "nine".to_string()];
    let mut ans = 0;
    for line in reader.lines() {
        let dat = line.unwrap();

        let mut a = 0;
        let mut b = 0;

        for (i, c) in dat.chars().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if a == 0 {
                    a = digit;
                }
                b = digit;
            }
            
            for (k, num) in nums.iter().enumerate() {
                if i < num.len() - 1 {
                    continue;
                }
                let mut flag = true;
                for j in 0..num.len() {
                    if dat.as_bytes()[i + 1 - num.len() + j] != num.as_bytes()[j] {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    let digit = (k + 1) as u32;
                    if a == 0 {
                        a = digit;
                    }
                    b = digit;
                }
            }
            
        }
        ans += a * 10 + b;
    }
    println!("{}", ans);
}
