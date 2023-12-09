use std::io::{self, BufRead};
use std::cmp;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut ans: i64 = 0;
    
    for line in reader.lines() {
        let dat = line.unwrap();
        let mut attempts = dat.split(": ");
        let _num: i64 = attempts.next().unwrap()[5..].parse().unwrap();
        let mut mx: HashMap<&str, i64> = HashMap::new();
        
        for game in attempts.next().unwrap_or_default().split("; ") {
            for dat in game.split(", ") {
                let mut parts = dat.split(" "); 
                let count: i64 = parts.next().unwrap().parse().unwrap();
                let color = parts.next().unwrap();
                mx.entry(color.clone()).and_modify(|e| *e = cmp::max(*e, count)).or_insert(count);
            }
        }
        
        let mut now = 1;
        for color in ["blue", "red", "green"].iter() {
            if let Some(num) = mx.get(color) {
                now *= num;
            } else {
                now = 0;
            }
        }
        ans += now;
    }
    println!("{}", ans);
}
