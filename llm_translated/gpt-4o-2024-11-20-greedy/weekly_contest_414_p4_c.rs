use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::cmp::{max, min};

const INF: i32 = i32::MAX;
const N: usize = 50;

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct Solution {
    pos: Vec<Position>,
    dist: Vec<Vec<i32>>,
    n: usize,
}

impl Solution {
    fn new(n: usize) -> Self {
        Solution {
            pos: vec![Position { x: 0, y: 0 }; n + 1],
            dist: vec![vec![0; n]; n + 1],
            n,
        }
    }

    fn calculate_distances(&mut self) {
        let directions = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1),
        ];

        for i in 0..=self.n {
            for j in 0..self.n {
                if i == j {
                    continue;
                }

                let mut queue = VecDeque::new();
                queue.push_back(self.pos[i]);
                let mut seen = vec![vec![false; N]; N];
                seen[self.pos[i].x][self.pos[i].y] = true;
                let mut steps = 0;

                'bfs: while !queue.is_empty() {
                    for _ in 0..queue.len() {
                        let current = queue.pop_front().unwrap();
                        if current.x == self.pos[j].x && current.y == self.pos[j].y {
                            self.dist[i][j] = steps;
                            break 'bfs;
                        }
                        for &(dx, dy) in &directions {
                            let nx = current.x as isize + dx;
                            let ny = current.y as isize + dy;
                            if nx >= 0 && nx < N as isize && ny >= 0 && ny < N as isize {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                if !seen[nx][ny] {
                                    queue.push_back(Position { x: nx, y: ny });
                                    seen[nx][ny] = true;
                                }
                            }
                        }
                    }
                    steps += 1;
                }
            }
        }
    }
}

fn dfs(
    sol: &Solution,
    memo: &mut Vec<Vec<Vec<i32>>>,
    i: usize,
    m: u64,
    turn: usize,
) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if memo[i][m as usize][turn] != -1 {
        return memo[i][m as usize][turn];
    }

    let mut ans = if turn == 0 { 0 } else { INF };
    for k in 0..sol.n {
        if m & (1 << k) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, memo, k, next_m, 1 - turn);
            if turn == 0 {
                ans = max(ans, result);
            } else {
                ans = min(ans, result);
            }
        }
    }
    memo[i][m as usize][turn] = ans;
    ans
}

fn max_moves(kx: usize, ky: usize, positions: Vec<(usize, usize)>) -> i32 {
    let positions_size = positions.len();
    let mut sol = Solution::new(positions_size);
    for (i, &(x, y)) in positions.iter().enumerate() {
        sol.pos[i] = Position { x, y };
    }
    sol.pos[positions_size] = Position { x: kx, y: ky };

    sol.calculate_distances();

    let mut memo = vec![vec![vec![-1; 2]; 1 << positions_size]; positions_size + 1];
    dfs(&sol, &mut memo, positions_size, 0, 0)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let kx: usize = first_iter.next().unwrap().parse().unwrap();
    let ky: usize = first_iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let positions_size: usize = second_line.parse().unwrap();

    let mut positions = Vec::new();
    for _ in 0..positions_size {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        positions.push((x, y));
    }

    let result = max_moves(kx, ky, positions);
    println!("{}", result);
}