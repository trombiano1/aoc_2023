use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut gal: Vec<(usize, usize)> = Vec::new();
    for line in reader.lines() {
        let dat = line.unwrap();
        map.push(dat.chars().collect());
    }

    let n = map.len();
    let m = map[0].len();

    let mut v = vec![false; n];
    let mut h = vec![false; m];
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if c == &'#' {
                v[i] = true;
                h[j] = true;
                gal.push((i, j));
            }
        }
    }

    let mut v_r = vec![0; n + 1];
    let mut h_r = vec![0; m + 1];
    for (i, _) in v.iter().enumerate() {
        if v[i] {
            v_r[i + 1] = v_r[i];
        } else {
            v_r[i + 1] = v_r[i] + 1;
        }
    }

    for (j, _) in h.iter().enumerate() {
        if h[j] {
            h_r[j + 1] = h_r[j];
        } else {
            h_r[j + 1] = h_r[j] + 1;
        }
    }

    let mut ans: u64 = 0;
    for k in 0..gal.len() - 1 {
        for l in k..gal.len() {
            let i1 = std::cmp::min(gal[k].0, gal[l].0);
            let i2 = std::cmp::max(gal[k].0, gal[l].0);
            let j1 = std::cmp::min(gal[k].1, gal[l].1);
            let j2 = std::cmp::max(gal[k].1, gal[l].1);
            ans += ((i2 - i1)
                + (j2 - j1)
                + (v_r[i2] - v_r[i1]) * 999999
                + (h_r[j2] - h_r[j1]) * 999999) as u64;
        }
    }

    println!("{}", ans);
}
