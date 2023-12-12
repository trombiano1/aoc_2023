use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut ans: i64 = 0;
    
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    
    let mut count: Vec<i64> = vec![1; lines.len().try_into().unwrap()];
    count[0] = 1;

    for (i, line) in lines.iter().enumerate() {
        let card = line.split(": ");
        let mut info = card.skip(1).next().unwrap().split(" | ");
        let mut now: usize = 0;
        
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
                now += 1;
            }
        }
        
        for j in 1..=now {
            if i + j < lines.len() {
                count[i + j] += count[i];
            }
        } 
    }
    for c in count {
        ans += c;
    }
    println!("{}", ans);
}
