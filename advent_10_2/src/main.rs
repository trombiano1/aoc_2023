use std::io::{self, BufRead};
const inf: usize = usize::MAX;
use std::collections::VecDeque;

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

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut s = (0, 0);
    
    let mut dots = 0;
    for (i, line) in reader.lines().enumerate() {
        let dat = ".".to_owned() + &line.unwrap() + &".".to_owned();
        if i == 0 {
            let vec_of_dots: Vec<char> = vec!['.'; dat.len()];
            board.push(vec_of_dots);
        }
        board.push(dat.chars().collect());
        for (j, c) in board.last().unwrap().iter().enumerate() {
            if c == &'S' {
                s = (i + 1, j + 1);
            }
            if c == &'.' {
                dots += 1;
            }
        }
    }

    let vec_of_dots: Vec<char> = vec!['.'; board[0].len()];
    board.push(vec_of_dots);   
    dots += board[0].len() * 2;

    // for l in &board {
    //     for c in l {
    //         print!("{} ", c);
    //     }
    //     println!("");
    // }

    let mut dist: Vec<Vec<usize>> = vec![vec![inf; board[0].len() * 2]; board.len() * 2];
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_back((s.0, s.1));
    dist[2 * s.0][2 * s.1] = 0;

    let n = board.len();
    let m = board[0].len();

    while let Some((i, j)) = que.pop_front() {
        let now = dist[2 * i][2 * j] + 1;
        // println!("{}: {}, {} ({})", board[i][j], i, j, now);
        
        match board[i][j] {
            '|' => {
                if i > 0 && dist[2 * i - 2][2 * j] == inf {
                    que.push_back((i - 1, j));
                    dist[2 * i - 1][2 * j] = now;
                    dist[2 * i - 2][2 * j] = now;
                } else if i > 0 {
                    dist[2 * i - 1][2 * j] = now;
                }
                if i + 1 < n && dist[2 * i + 2][2 * j] == inf {
                    que.push_back((i + 1, j));
                    dist[2 * i + 1][2 * j] = now;
                    dist[2 * i + 2][2 * j] = now;
                } else if i + 1 < n {
                    dist[2 * i + 1][2 * j] = now;
                }
            },
            '-' => {
                if j > 0 && dist[2 * i][2 * j - 2] == inf {
                    que.push_back((i, j - 1));
                    dist[2 * i][2 * j - 1] = now;
                    dist[2 * i][2 * j - 2] = now;
                } else if j > 0 {
                    dist[2 * i][2 * j - 1] = now;
                }
                if j + 1 < m && dist[2 * i][2 * j + 2] == inf {
                    que.push_back((i, j + 1));
                    dist[2 * i][2 * j + 1] = now;
                    dist[2 * i][2 * j + 2] = now;
                } else if j + 1 < m {
                    dist[2 * i][2 * j + 1] = now;
                }
            },
            'L' => {
                if i > 0 && dist[2 * i - 2][2 * j] == inf {
                    que.push_back((i - 1, j));
                    dist[2 * i - 1][2 * j] = now;
                    dist[2 * i - 2][2 * j] = now;
                } else if i > 0 {
                    dist[2 * i - 1][2 * j] = now;
                }
                if j + 1 < m && dist[2 * i][2 * j + 2] == inf {
                    que.push_back((i, j + 1));
                    dist[2 * i][2 * j + 1] = now;
                    dist[2 * i][2 * j + 2] = now;
                } else if j + 1 < m {
                    dist[2 * i][2 * j + 1] = now;
                }
            },
            'J' => {
                if i > 0 && dist[2 * i - 2][2 * j] == inf {
                    que.push_back((i - 1, j));
                    dist[2 * i - 1][2 * j] = now;
                    dist[2 * i - 2][2 * j] = now;
                } else if i > 0 {
                    dist[2 * i - 1][2 * j] = now;
                }
                if j > 0 && dist[2 * i][2 * j - 2] == inf {
                    que.push_back((i, j - 1));
                    dist[2 * i][2 * j - 1] = now;
                    dist[2 * i][2 * j - 2] = now;
                } else if j > 0 {
                    dist[2 * i][2 * j - 1] = now;
                }
            },
            '7' => {
                if i + 1 < n && dist[2 * i + 2][2 * j] == inf {
                    que.push_back((i + 1, j));
                    dist[2 * i + 1][2 * j] = now;
                    dist[2 * i + 2][2 * j] = now;
                } else if i + 1 < n {
                    dist[2 * i + 1][2 * j] = now;
                }
                if j > 0 && dist[2 * i][2 * j - 2] == inf {
                    que.push_back((i, j - 1));
                    dist[2 * i][2 * j - 1] = now;
                    dist[2 * i][2 * j - 2] = now;
                } else if j > 0 {
                    dist[2 * i][2 * j - 1] = now;
                }
            },
            'F' => {
                if i + 1 < n && dist[2 * i + 2][2 * j] == inf {
                    que.push_back((i + 1, j));
                    dist[2 * i + 1][2 * j] = now;
                    dist[2 * i + 2][2 * j] = now;
                } else if i + 1 < n {
                    dist[2 * i + 1][2 * j] = now;
                }
                if j + 1 < m && dist[2 * i][2 * j + 2] == inf {
                    que.push_back((i, j + 1));
                    dist[2 * i][2 * j + 1] = now;
                    dist[2 * i][2 * j + 2] = now;
                } else if j + 1 < m {
                    dist[2 * i][2 * j + 1] = now;
                }
            },
            'S' => {
                if i + 1 < n && match board[i + 1][j] {
                    '|' => true,
                    'L' => true,
                    'J' => true,
                    _ => false,
                } { que.push_back((i + 1, j)); dist[2 * i + 1][2 * j] = now; dist[2 * i + 2][2 * j] = now; }
                
                if i > 0 && match board[i - 1][j] {
                    '|' => true,
                    '7' => true,
                    'F' => true,
                    _ => false,
                } { que.push_back((i - 1, j)); dist[2 * i - 1][2 * j] = now; dist[2 * i - 2][2 * j] = now; }
                
                if j + 1 < m && match board[i][j + 1] {
                    '-' => true,
                    'J' => true,
                    '7' => true,
                    _ => false,
                } { que.push_back((i, j + 1)); dist[2 * i][2 * j + 1] = now; dist[2 * i][2 * j + 2] = now; }
                
                if j > 0 && match board[i][j - 1] {
                    '-' => true,
                    'L' => true,
                    'F' => true,
                    _ => false,
                } { que.push_back((i, j - 1)); dist[2 * i][2 * j - 1] = now; dist[2 * i][2 * j - 2] = now; }
            },
            _ => {},
        }
    }
    
    let mut uf = UnionFind::new(4 * n * m);
    for i in 0..2 * n {
        for j in 0..2 * m {
            if dist[i][j] != inf {
                continue;
            }
            if i + 1 < 2 * n && dist[i + 1][j] == inf {
                uf.unite(i * 2 * m + j, (i + 1) * 2 * m + j);
            }
            if i > 0 && dist[i - 1][j] == inf {
                uf.unite(i * 2 * m + j, (i - 1) * 2 * m + j);
            }
            if j + 1 < 2 * m && dist[i][j + 1] == inf {
                uf.unite(i * 2 * m + j, i * 2 * m + (j + 1));
            }
            if j > 0 && dist[i][j - 1] == inf {
                uf.unite(i * 2 * m + j, i * 2 * m + (j - 0));
            }
        
        }
    }

    let mut ans = 0;
    for i in 0..2 * n {
        for j in 0..2 * m {
            if i % 2 == 0 && j % 2 == 0 && dist[i][j] == inf && !uf.issame(0, i * 2 * m + j) {
                // println!("{} {} {} {}", i / 2, j / 2, board[i / 2][j / 2], uf.root(i * 2 * m + j));
                ans += 1;
            }
        }
    }

    // for (i, line) in dist.iter().enumerate() {
    //     for (j, d) in line.iter().enumerate() {
    //         if i % 2 != 0 || j % 2 != 0 {
    //             continue;
    //         }
    //         print!("{:3} ", uf.root(i * 2 * m + j));
    //         // if dist[i][j] == inf {
    //         //     print!("   ");
    //         // } else {
    //         //     print!("{:3}", dist[i][j]);
    //         // }
    //     }
    //     println!("");
    // }


    println!("{}", ans);
}


// vector<long long> dist(n, -1);
// queue<int> que;

// dist[0] = 0;
// que.push(0);

// while (!que.empty()) {
// 	int v = que.front();
// 	que.pop();

// 	for (long long nv : g[v]) {
// 		if (dist[nv] != -1) {
//             continue;
//         }

// 		dist[nv] = dist[v] + 1;
// 		que.push(nv);
// 	}
// }