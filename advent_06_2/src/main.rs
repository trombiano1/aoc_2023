use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut time: i64 = 0;
    let mut dist: i64 = 0;

    for (i, line) in reader.lines().enumerate() {
        let dat = line.unwrap();
        if i == 0 {
            let replaced_line = dat.replace(' ', "");
            let parts = replaced_line.split(":");
            time = parts.skip(1).next().unwrap().parse().unwrap();
        }
        if i == 1 {
            let replaced_line = dat.replace(' ', "");
            let parts = replaced_line.split(":");
            dist = parts.skip(1).next().unwrap().parse().unwrap();
        }
    }

    let det: f64 = f64::sqrt(((time * time) - 4 * dist) as f64);

    let mn_f = (time as f64 - det) as f64 / 2 as f64;
    let mx_f = (time as f64 + det) as f64 / 2 as f64;

    let mn = if mn_f.fract() == 0.0 {
        (mn_f + 1.0) as i64
    } else {
        mn_f.ceil() as i64
    };
    let mx = if mx_f.fract() == 0.0 {
        (mx_f - 1.0) as i64
    } else {
        mx_f.floor() as i64
    };

    println!("{}", mx - mn + 1);
}
