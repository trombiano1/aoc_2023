use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut ans = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut a: Option<u32> = None;
                let mut b: Option<u32> = None;
                
                for c in line.chars() {
                    if c.is_digit(10) {
                        let digit = c.to_digit(10).unwrap();
                        if a.is_none() {
                            a = Some(digit);
                        }
                        b = Some(digit);
                    }
                }
                if let (Some(a_val), Some(b_val)) = (a, b) {
                    ans += a_val * 10 + b_val;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
    println!("{}", ans);
}
