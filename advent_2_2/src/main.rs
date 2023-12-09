use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut ans: i32 = 0;
    
    let mut limits = HashMap::new();
    limits.insert(String::from("red"), 12);
    limits.insert(String::from("green"), 13);
    limits.insert(String::from("blue"), 14);
    
    for line in reader.lines() {
        let dat = line.unwrap();
        let mut attempts = dat.split(": ");
        let num: i32 = attempts.next().unwrap()[5..].parse().unwrap();
        let mut ok = true;
        for game in attempts.next().unwrap_or_default().split("; ") {
            if !ok {
                break;
            }
            for dat in game.split(", ") {
                let mut parts = dat.split(" "); 
                let count: i32 = parts.next().unwrap().parse().unwrap();
                let color = parts.next().unwrap();
                
                if count > limits[color] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            ans += num;
        }
    }
    println!("{}", ans);
}
