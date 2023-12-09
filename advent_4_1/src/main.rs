use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut ans: i64 = 0;
    for line in reader.lines() {
        let dat = line.unwrap();
        let card = dat.split(": ");
        let mut info = card.skip(1).next().unwrap().split(" | ");
        let mut now: i64 = 0;
        
        let win_str = info.next().unwrap();
        let mut win: HashSet<i32> = HashSet::new();
        for num_str in win_str.split(" ") {
            if num_str == "" {
                continue;
            }
            let num: i32 = num_str.parse().unwrap();
            win.insert(num);
        }
        
        let my_str = info.next().unwrap();
        for num_str in my_str.split(" ") {
            if num_str == "" {
                continue;
            }
            let num: i32 = num_str.parse().unwrap();
            if win.contains(&num) {
                if now == 0 {
                    now += 1;
                } else {
                    now *= 2;
                }
            }
        }
        ans += now;
    }
    println!("{}", ans);
}
