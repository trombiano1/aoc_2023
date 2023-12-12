use std::io::{self, BufRead};
use std::collections::HashMap;

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
    
    let mut now = "AAA".to_string();
    let mut ans = 0;
    loop {
        for c in inst.chars() {
            if c == 'R' {
                now = g[&now].1.clone();
            } else {
                now = g[&now].0.clone();
            }
            ans += 1;
        }
        if now == "ZZZ".to_string() {
            break;
        }
    }

    println!("{}", ans);
}
