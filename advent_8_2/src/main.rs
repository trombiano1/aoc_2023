use std::io::{self, BufRead};
use std::collections::HashMap;
use num_integer::Integer;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut g: HashMap<String, (String, String)> = HashMap::new();
    let mut inst: String = "".to_string();
    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            inst = line.unwrap();
            continue;
        }
        
        if i == 1 {
            continue;
        }
        
        let dat = line.unwrap();
        let frm = &dat[0..3];
        let left = &dat[7..10];
        let right = &dat[12..15];
        
        g.insert(frm.to_string(), (left.to_string(), right.to_string()));
    }
    
    let mut ghosts: Vec<String> = Vec::new();
    for (s, _) in &g {
        if s.chars().nth(2).unwrap() == 'A' {
            ghosts.push(s.to_string());
        }
    }

    let mut ans = 1;
    for ghost in &ghosts {
        let mut count = 0;
        let mut now = ghost.clone();
        loop {
            for c in inst.chars() {
                if c == 'L' {
                    now = g[&now].0.clone();
                } else {
                    now = g[&now].1.clone();
                }
                count += 1;
            }
            if now.chars().nth(2).unwrap() == 'Z' {
                break;
            }
        }
        ans = ans as i64 * count as i64 / ans.gcd(&count);
    }
    println!("{}", ans);
}
