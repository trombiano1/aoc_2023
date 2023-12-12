use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut ans = 0;
    for line in reader.lines() {
        let dat = line.unwrap();
        let mut parts = dat.split(" ");
        let map = parts.next().unwrap();
        let counts_str = parts.next().unwrap();
        let mut counts: Vec<usize> = Vec::new();
        for count_str in counts_str.split(",") {
            counts.push(count_str.parse().unwrap());
        }
        for mask in 0..(1 << map.len()) {
            let mut now = String::new();
            for i in 0..map.len() {
                if mask & (1 << i) == 0 {
                    now += ".";
                } else {
                    now += "#";
                }
            }
            let mut now_counts = Vec::new();
            let mut l = 0;
            for c in now.chars() {
                if c == '.' {
                    if l != 0 {
                        now_counts.push(l);
                    }
                    l = 0;
                } else {
                    l += 1;
                }
            }
            if l != 0 {
                now_counts.push(l);
            }

            if now_counts != counts {
                continue;
            }

            let mut ok = true;
            for (c, m) in now
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .zip(map.chars().collect::<Vec<char>>().iter())
            {
                match (c, m) {
                    ('.', '#') => ok = false,
                    ('#', '.') => ok = false,
                    (_, _) => {}
                }
            }

            if ok {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
