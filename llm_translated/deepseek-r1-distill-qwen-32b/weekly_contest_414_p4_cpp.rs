use std::collections::{HashMap, VecDeque};

struct Solution {
    dis: Vec<Vec<Vec<i32>>>,
    positions: Vec<Vec<i32>>,
    n: usize,
    memo: HashMap<(usize, usize), i32>,
}

impl Solution {
    const DIRS: [(i32, i32); 8] = [
        (2, 1), (1, 2), (-1, 2), (-2, 1),
        (-2, -1), (-1, -2), (1, -2), (2, -1),
    ];

    fn new(n: usize, positions: Vec<Vec<i32>>) -> Self {
        let dis = vec![vec![vec![-1; 50]; 50]; n];
        Self {
            dis,
            positions,
            n,
            memo: HashMap::new(),
        }
    }

    fn precompute_dis(&mut self) {
        for i in 0..self.n {
            let (px, py) = (self.positions[i][0], self.positions[i][1]);
            for x in 0..50 {
                for y in 0..50 {
                    self.dis[i][x][y] = -1;
                }
            }
            self.dis[i][px as usize][py as usize] = 0;
            let mut q = VecDeque::new();
            q.push_back((px, py));
            let mut step = 1;
            while !q.is_empty() {
                let mut tmp = VecDeque::new();
                while let Some((x, y)) = q.pop_front() {
                    for (dx, dy) in Self::DIRS {
                        let new_x = x + dx;
                        let new_y = y + dy;
                        if new_x >= 0 && new_x < 50 && new_y >= 0 && new_y < 50 {
                            if self.dis[i][new_x as usize][new_y as usize] == -1 {
                                self.dis[i][new_x as usize][new_y as usize] = step;
                                tmp.push_back((new_x, new_y));
                            }
                        }
                    }
                }
                q = tmp;
                step += 1;
            }
        }
    }

    fn dfs(&mut self, i: usize, mask: usize) -> i32 {
        if mask == 0 {
            return 0;
        }
        if let Some(&res) = self.memo.get(&(i, mask)) {
            return res;
        }
        let x = self.positions[i][0] as usize;
        let y = self.positions[i][1] as usize;
        let u = (1 << self.n) - 1;
        let remaining = (u ^ mask).count_ones();
        let is_alice = (remaining % 2) == 0;
        let mut res = if is_alice { i32::MIN } else { i32::MAX };
        for j in 0..self.n {
            if (mask & (1 << j)) != 0 {
                let new_mask = mask ^ (1 << j);
                let step = self.dis[j][x][y];
                let val = self.dfs(j, new_mask) + step;
                if is_alice {
                    if val > res {
                        res = val;
                    }
                } else {
                    if val < res {
                        res = val;
                    }
                }
            }
        }
        self.memo.insert((i, mask), res);
        res
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let kx: i32 = parts.next().unwrap().parse().unwrap();
    let ky: i32 = parts.next().unwrap().parse().unwrap();
    let n: usize = parts.next().unwrap().parse().unwrap();

    let mut positions = Vec::with_capacity(n + 1);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        positions.push(vec![x, y]);
    }
    positions.push(vec![kx, ky]);

    let mut solution = Solution::new(n, positions);
    solution.precompute_dis();
    let result = solution.dfs(n, (1 << n) - 1);
    println!("{}", result);
}