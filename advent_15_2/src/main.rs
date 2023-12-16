use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut boxes: HashMap<usize, HashMap<String, (usize, usize)>> = HashMap::new();
    for line in reader.lines() {
        let dat: String= line.unwrap();
        for (i, seq) in dat.split(",").enumerate() {
            let label: String;
            let focal: Option<usize>;
            if seq.chars().last().unwrap() == '-' {
                label= seq[0..seq.len() - 1].to_owned();
                focal = None;
            } else {
                let mut parts = seq.split("=");
                label = parts.next().unwrap().to_owned();
                focal = Some(parts.next().unwrap().parse::<usize>().unwrap());
            }

            let mut hash: usize = 0;
            for c in label.chars() {
                hash += (c as u8) as usize;
                hash *= 17;
                hash %= 256;
            }

            if focal.is_none() {
                if boxes.contains_key(&hash) {
                    if boxes[&hash].contains_key(&label) {
                        boxes.get_mut(&hash).unwrap().remove(&label);
                    }
                }
            } else {
                if !boxes.contains_key(&hash) {
                    boxes.insert(hash.clone(), HashMap::new());
                }
                if boxes[&hash].contains_key(&label) {
                    let j = boxes[&hash][&label].1; 
                    boxes.get_mut(&hash).unwrap().insert(label.clone(), (focal.unwrap(), j));
                } else {
                    boxes.get_mut(&hash).unwrap().insert(label.clone(), (focal.unwrap(), i as usize));
                }
            }
        }
    }

    let mut ans = 0;
    for (hash, map) in &boxes {
        let mut lenses: Vec<_> = map.values().collect();
        lenses.sort_by(|i, j| i.1.cmp(&j.1)); 
        for (j, lens) in lenses.iter().enumerate() {
            ans += (hash + 1) * (j + 1) * lens.0;
        }
    }
    println!("{}", ans);
}
