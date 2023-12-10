use std::io::{self, BufRead};
const inf: usize = usize::MAX;
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut s = (0, 0);
    
    for (i, line) in reader.lines().enumerate() {
        board.push(line.unwrap().chars().collect());
        for (j, c) in board.last().unwrap().iter().enumerate() {
            if c == &'S' {
                s = (i, j);
            }
        }
    }
    
    let mut dist: Vec<Vec<usize>> = vec![vec![inf; board[0].len()]; board.len()];
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();
    que.push_back((s.0, s.1));
    dist[s.0][s.1] = 0;

    let n = board.len();
    let m = board[0].len();

    while let Some((i, j)) = que.pop_front() {
        let now = dist[i][j] + 1;
        
        match board[i][j] {
            '|' => {
                if i - 1 >= 0 && dist[i - 1][j] == inf { que.push_back((i - 1, j)); dist[i - 1][j] = now; }
                if i + 1 < n && dist[i + 1][j] == inf { que.push_back((i + 1, j)); dist[i + 1][j] = now; }
            },
            '-' => {
                if j - 1 >= 0 && dist[i][j - 1] == inf { que.push_back((i, j - 1)); dist[i][j - 1] = now; }
                if j + 1 < m && dist[i][j + 1] == inf { que.push_back((i, j + 1)); dist[i][j + 1] = now; }
            },
            'L' => {
                if i - 1 >= 0 && dist[i - 1][j] == inf { que.push_back((i - 1, j)); dist[i - 1][j] = now; }
                if j + 1 < m && dist[i][j + 1] == inf { que.push_back((i, j + 1)); dist[i][j + 1] = now; }
            },
            'J' => {
                if i - 1 >= 0 && dist[i - 1][j] == inf { que.push_back((i - 1, j)); dist[i - 1][j] = now; }
                if j - 1 >= 0 && dist[i][j - 1] == inf { que.push_back((i, j - 1)); dist[i][j - 1] = now; }
            },
            '7' => {
                if i + 1 < n && dist[i + 1][j] == inf { que.push_back((i + 1, j)); dist[i + 1][j] = now; }
                if j - 1 >= 0 && dist[i][j - 1] == inf { que.push_back((i, j - 1)); dist[i][j - 1] = now; }
            },
            'F' => {
                if i + 1 < n && dist[i + 1][j] == inf { que.push_back((i + 1, j)); dist[i + 1][j] = now; }
                if j + 1 < m && dist[i][j + 1] == inf { que.push_back((i, j + 1)); dist[i][j + 1] = now; }
            },
            'S' => {
                if i + 1 < n && match board[i + 1][j] {
                    '|' => true,
                    'L' => true,
                    'J' => true,
                    _ => false,
                } { que.push_back((i + 1, j)); dist[i + 1][j] = now; }
                
                if i > 0 && match board[i - 1][j] {
                    '|' => true,
                    '7' => true,
                    'F' => true,
                    _ => false,
                } { que.push_back((i - 1, j)); dist[i - 1][j] = now; }
                
                if j + 1 < m && match board[i][j + 1] {
                    '-' => true,
                    'J' => true,
                    '7' => true,
                    _ => false,
                } { que.push_back((i, j + 1)); dist[i][j + 1] = now; }
                
                if j > 0 && match board[i][j - 1] {
                    '-' => true,
                    'L' => true,
                    'F' => true,
                    _ => false,
                } { que.push_back((i, j - 1)); dist[i][j - 1] = now; }
            },
            _ => {},
        }
    }
    
    let mut ans = 0;
    for line in dist {
        for d in line {
            if d != inf {
                ans = std::cmp::max(ans, d);
            }
        }
    }
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