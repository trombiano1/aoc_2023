use std::io::{self, BufRead};

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }
}

fn compress(c: &mut Vec<(isize, isize)>) -> Vec<isize> {
    let mut vals: Vec<isize> = Vec::new();

    let n = c.len();
    for i in 0..n {
        for d in 0..1 {
            let tc1 = c[i].0 + d;
            let tc2 = c[i].1 + d;
            vals.push(tc1);
            vals.push(tc2);
        }
    }
    vals.sort();
    vals.dedup();
    for i in 0..n {
        c[i] = (
            vals.binary_search(&c[i].0).unwrap_or_else(|x| x) as isize,
            vals.binary_search(&c[i].1).unwrap_or_else(|x| x) as isize,
        );
    }
    return vals;
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut plans = Vec::new();
    let mut i: isize = 0;
    let mut j: isize = 0;

    plans.push((isize::MIN, isize::MAX));
    plans.push((0, 0));

    for line in reader.lines() {
        let dat = line.unwrap();
        let mut parts = dat.split(' ');

        let _ = parts.next();
        let _ = parts.next();

        let color = parts.next().unwrap();
        let dir = color[7..8].parse::<usize>().unwrap() as usize;
        let c = isize::from_str_radix(&color[2..7], 16).unwrap();

        i = i + dirs[dir].0 * c;
        j = j + dirs[dir].1 * c;

        plans.push((i, j));
        plans.push((i - 1, j - 1));
    }

    let vals = compress(&mut plans);
    let mut map: Vec<Vec<bool>> = vec![vec![false; 2 * vals.len() + 1]; 2 * vals.len() + 1];

    plans.remove(0);
    i = plans[0].0;
    j = plans[0].1;

    for (idx, (ni, nj)) in plans.iter().enumerate() {
        if idx % 2 == 0 {
            continue;
        }
        if ni != &i {
            for k in std::cmp::min(2 * i, 2 * ni)..=std::cmp::max(2 * i, 2 * ni) {
                map[k as usize][2 * j as usize] = true;
            }
            i = ni.clone();
        }
        if nj != &j {
            for k in std::cmp::min(2 * j, 2 * nj)..=std::cmp::max(2 * j, 2 * nj) {
                map[2 * i as usize][k as usize] = true;
            }
            j = nj.clone();
        }
    }

    let n = map.len();
    let m = map[0].len();
    let mut uf = UnionFind::new(4 * n * m);

    for i in 0..n {
        for j in 0..m {
            if map[i][j] == true {
                continue;
            }
            if i + 1 < n && map[i + 1][j] == false {
                uf.unite(i * 2 * m + j, (i + 1) * 2 * m + j);
            }
            if i > 0 && map[i - 1][j] == false {
                uf.unite(i * 2 * m + j, (i - 1) * 2 * m + j);
            }
            if j + 1 < m && map[i][j + 1] == false {
                uf.unite(i * 2 * m + j, i * 2 * m + (j + 1));
            }
            if j > 0 && map[i][j - 1] == false {
                uf.unite(i * 2 * m + j, i * 2 * m + (j - 0));
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if i % 2 == 0 && j % 2 == 0 && !uf.issame(0, i * 2 * m + j) {
                ans += (vals[i / 2] - vals[i / 2 - 1]) * (vals[j / 2] - vals[j / 2 - 1])
            }
        }
    }
    println!("{}", ans);
}
