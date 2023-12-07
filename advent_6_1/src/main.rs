use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut time: Vec<i32> = Vec::new();
    let mut dist: Vec<i32> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if i == 0 {
                    let parts = line.split(": ");
                    let info_str = parts.skip(1).next().unwrap();
                    for time_str in info_str.split(" ") {
                        if time_str == "" {
                            continue;
                        }
                        time.push(time_str.parse().unwrap());
                    }
                }

                if i == 1 {
                    let parts = line.split(": ");
                    let info_str = parts.skip(1).next().unwrap();
                    for dist_str in info_str.split(" ") {
                        if dist_str == "" {
                            continue;
                        }
                        dist.push(dist_str.parse().unwrap());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    let mut ans: i64 = 1;
    for (t, d) in time.iter().zip(dist.iter()) {
        let det: f64 = f64::sqrt(((t * t) - 4 * d) as f64);
        let mn_f = (*t as f64 - det) as f64 / 2 as f64;
        let mx_f = (*t as f64 + det) as f64 / 2 as f64;
        let mn = if mn_f.fract() == 0.0 {
            (mn_f + 1.0) as i64
        } else {
            mn_f.ceil() as i64
        };
        let mx = if mx_f.fract() == 0.0 {
            (mx_f - 1.0) as i64
        } else {
            mx_f.floor() as i64
        };
        ans *= mx - mn + 1;
    }

    println!("{}", ans);
}
