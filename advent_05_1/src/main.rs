use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let dat = line.unwrap();
        if i == 0 {
            let parts = dat.split(": ");
            let seeds_str = parts.skip(1).next().unwrap();
            for seed_str in seeds_str.split(" ") {
                seeds.push(seed_str.parse().unwrap());
            }
            continue;
        }
        
        if dat == "" {
            continue;
        }
        
        if dat.chars().last().unwrap() == ':' {
            maps.push(Vec::new());
            continue;
        }
        
        let mut nums = dat.split(" ");
        let dst: i64 = nums.next().unwrap().parse().unwrap();
        let src: i64 = nums.next().unwrap().parse().unwrap();
        let range: i64 = nums.next().unwrap().parse().unwrap();
        
        maps.last_mut().unwrap().push((dst, src, range));
    }
    
    let mut mn = std::i64::MAX;
    for seed in seeds {
        let mut loc = seed;
        for map in &maps {
            for (dst, src, rng) in map {
                if src <= &loc && &loc < &(src + rng) {
                    loc += dst - src;
                    break;
                }
            }
        }
        mn = std::cmp::min(mn, loc.try_into().unwrap());
    }
    println!("{}", mn);
}
