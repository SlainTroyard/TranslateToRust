use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io;

const INF: i32 = i32::MAX;
const N: usize = 50;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct Solution {
    pos: Vec<Position>, // Position[N + 1]
    dist: Vec<Vec<i32>>, // dist[N + 1][N]
    n: usize,
}

struct Queue {
    queue: VecDeque<Position>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            queue: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, p: Position) {
        self.queue.push_back(p);
    }

    fn dequeue(&mut self) -> Option<Position> {
        self.queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn is_valid(x: usize, y: usize) -> bool {
    x < N && y < N
}

fn calculate_distances(sol: &mut Solution) {
    let directions: [[i32; 2]; 8] = [
        [-2, -1],
        [-2, 1],
        [-1, -2],
        [-1, 2],
        [1, -2],
        [1, 2],
        [2, -1],
        [2, 1],
    ];

    for i in 0..=sol.n {
        for j in 0..sol.n {
            if i == j {
                continue;
            }
            let mut queue = Queue::new();
            queue.enqueue(sol.pos[i]);
            let mut seen = vec![vec![false; N]; N];
            seen[sol.pos[i].x][sol.pos[i].y] = true;
            let mut steps = 0;

            'outer: while !queue.is_empty() {
                let size = queue.queue.len();
                for _s in 0..size {
                    if let Some(current) = queue.dequeue() {
                        if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                            sol.dist[i][j] = steps;
                            break 'outer;
                        }
                        for d in 0..8 {
                            let nx = current.x as i32 + directions[d][0];
                            let ny = current.y as i32 + directions[d][1];
                            if nx >= 0 && nx < N as i32 && ny >= 0 && ny < N as i32 {
                                let nx = nx as usize;
                                let ny = ny as usize;
                                if is_valid(nx, ny) && !seen[nx][ny] {
                                    queue.enqueue(Position { x: nx, y: ny });
                                    seen[nx][ny] = true;
                                }
                            }
                        }
                    }
                }
                steps += 1;
            }
        }
    }
}

fn dfs(sol: &Solution, i: usize, m: i64, turn: usize, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    if m == (1i64 << sol.n) - 1 {
        return 0;
    }
    if memo[i][m as usize][turn] != -1 {
        return memo[i][m as usize][turn];
    }

    let mut ans = if turn == 0 { 0 } else { INF };
    for k in 0..sol.n {
        if (m & (1i64 << k)) == 0 {
            let next_m = m | (1i64 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                ans = max(result, ans);
            } else {
                ans = min(result, ans);
            }
        }
    }
    memo[i][m as usize][turn] = ans;
    ans
}

fn max_moves(kx: usize, ky: usize, positions: &Vec<Vec<usize>>, positions_size: usize) -> i32 {
    let mut sol = Solution {
        pos: vec![Position { x: 0, y: 0 }; positions_size + 1],
        dist: vec![vec![0; positions_size]; positions_size + 1],
        n: positions_size,
    };

    for i in 0..positions_size {
        sol.pos[i].x = positions[i][0];
        sol.pos[i].y = positions[i][1];
    }
    sol.pos[positions_size] = Position { x: kx, y: ky };

    let mut memo = vec![
        vec![vec![-1; 2]; 1 << positions_size];
        N + 1
    ];

    calculate_distances(&mut sol);
    dfs(&sol, sol.n, 0, 0, &mut memo)
}

fn main() -> io::Result<()> {
    let mut kx = String::new();
    io::stdin().read_line(&mut kx)?;
    let mut kx_iter = kx.trim().split_whitespace();
    let kx_val: usize = kx_iter.next().unwrap().parse().unwrap();
    let ky_val: usize = kx_iter.next().unwrap().parse().unwrap();

    let mut positions_size_str = String::new();
    io::stdin().read_line(&mut positions_size_str)?;
    let positions_size: usize = positions_size_str.trim().parse().unwrap();

    let mut positions: Vec<Vec<usize>> = Vec::new();
    for _ in 0..positions_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let values: Vec<usize> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        positions.push(values);
    }

    let result = max_moves(kx_val, ky_val, &positions, positions_size);
    println!("{}", result);

    Ok(())
}