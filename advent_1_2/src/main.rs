use proconio::input;

fn main() {
    let nums: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string(),
                               "four".to_string(), "five".to_string(), "six".to_string(),
                               "seven".to_string(), "eight".to_string(), "nine".to_string()];
    let mut ans = 0;
    loop {
        input! {
            line: String,
        }

        if line.is_empty() {
            break;
        }

        let mut a: Option<u32> = None;
        let mut b: Option<u32> = None;
        
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if a.is_none() {
                    a = Some(digit);
                }
                b = Some(digit);
            }
            
            for (k, num) in nums.iter().enumerate() {
                if i < num.len() - 1 {
                    continue;
                }
                let mut flag = true;
                for j in 0..num.len() {
                    if line.as_bytes()[i + 1 - num.len() + j] != num.as_bytes()[j] {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    let digit = (k + 1) as u32;
                    if a.is_none() {
                        a = Some(digit);
                    }
                    b = Some(digit);
                }
            }
            
        }
        if let (Some(a_val), Some(b_val)) = (a, b) {
            ans += a_val * 10 + b_val;
        }
    }
    println!("{}", ans);
}
