use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut ans = 0;
    for line in reader.lines() {
        let dat: String= line.unwrap();
        for seq in dat.split(",") {
            let mut hash = 0;
            for c in seq.chars() {
                hash += (c as u8) as i32;
                hash *= 17;
                hash %= 256;
            }
            ans += hash;
        }
    }
    println!("{}", ans);
}
