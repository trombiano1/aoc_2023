use std::collections::HashMap;
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

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut map: Vec<Vec<bool>> = vec![vec![false; 10000]; 10000];

    let mut i: usize = 5000;
    let mut j: usize = 5000;

    let mut dirs: HashMap<char, (i32, i32)> = HashMap::new();
    dirs.insert('R', (0, 1));
    dirs.insert('D', (1, 0));
    dirs.insert('L', (0, -1));
    dirs.insert('U', (-1, 0));
        let dat = line.unwrap();
        let mut parts = dat.split(' ');

        let dir: char = parts.next().unwrap().parse().unwrap();
        let c: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..c {
            for _ in 0..2 {
                i = (i as i32 + dirs[&dir].0) as usize;
                j = (j as i32 + dirs[&dir].1) as usize;

                map[i][j] = true;
            }
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
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
