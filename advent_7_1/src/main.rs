use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut mp: HashMap<char, usize> = HashMap::new();
    let char_strength = "23456789TJQKA";
    for (i, c) in char_strength.chars().enumerate() {
        mp.insert(c, i);
    }

    let mut cards: Vec<Vec<(String, i32)>> = vec![vec![]; 7];
    for line in reader.lines() {
        let dat = line.unwrap();
        let mut parts = dat.split(" ");
        let card = parts.next().unwrap().to_string();
        let bid: i32 = parts.next().unwrap().parse().unwrap();
        
        let mut alph = vec![0; 13];
        for c in card.chars() {
            alph[mp[&c]] += 1;
        }
        
        let mut max = 0;
        let mut second_max = 0;
        for a in alph {
            if a > max {
                second_max = max;
                max = a;
            } else if a > second_max {
                second_max = a;
            }
        }
        
        let idx = |max, second_max| match (max, second_max) {
            (5, _) => 6 as usize,
            (4, _) => 5 as usize,
            (3, 2) => 4 as usize,
            (3, 1) => 3 as usize,
            (2, 2) => 2 as usize,
            (2, 1) => 1 as usize,
            (1, _) => 0 as usize,
            _ => panic!("err"),
        };
        
        cards[idx(max, second_max)].push((card, bid));
    }
    
    let mut c = 0;
    let mut ans: i64 = 0;
    for mut card_type in cards {
        card_type.sort_by(|i, j| {
            for (ci, cj) in i.0.chars().zip(j.0.chars()) {
                if ci != cj {
                    return mp[&ci].cmp(&mp[&cj]);
                }
            }
            std::cmp::Ordering::Equal
        });
        for card in card_type {
            c += 1;
            ans += card.1 as i64 * c as i64;
        }
    }

    println!("{}", ans);
}
