use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io;

const INF: i32 = i32::MAX;
const N: usize = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

struct Solution {
    pos: [Position; N + 1],
    dist: [[i32; N]; N + 1],
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

fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32
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
            let mut seen = [[false; N]; N];
            seen[sol.pos[i].x as usize][sol.pos[i].y as usize] = true;
            let mut steps = 0;

            'outer: while !queue.is_empty() {
                let size = queue.queue.len();
                for _ in 0..size {
                    if let Some(current) = queue.dequeue() {
                        if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                            sol.dist[i][j] = steps;
                            break 'outer;
                        }
                        for d in 0..8 {
                            let nx = current.x + directions[d][0];
                            let ny = current.y + directions[d][1];
                            if is_valid(nx, ny) && !seen[nx as usize][ny as usize] {
                                queue.enqueue(Position { x: nx, y: ny });
                                seen[nx as usize][ny as usize] = true;
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

fn max_moves(kx: i32, ky: i32, positions: &Vec<Vec<i32>>, positions_size: usize) -> i32 {
    let mut sol = Solution {
        pos: [Position { x: 0, y: 0 }; N + 1],
        dist: [[0; N]; N + 1],
        n: positions_size,
    };

    for i in 0..positions_size {
        sol.pos[i].x = positions[i][0];
        sol.pos[i].y = positions[i][1];
    }
    sol.pos[positions_size] = Position { x: kx, y: ky };

    let mut memo = vec![vec![vec![-1; 2]; 1 << positions_size]; N + 1];

    calculate_distances(&mut sol);
    dfs(&sol, sol.n, 0, 0, &mut memo)
}

fn main() -> io::Result<()> {
    let mut kx = String::new();
    io::stdin().read_line(&mut kx)?;
    let mut kx_ky = kx.trim().split_whitespace();
    let kx: i32 = kx_ky.next().unwrap().parse().unwrap();
    let ky: i32 = kx_ky.next().unwrap().parse().unwrap();

    let mut positions_size_str = String::new();
    io::stdin().read_line(&mut positions_size_str)?;
    let positions_size: usize = positions_size_str.trim().parse().unwrap();

    let mut positions: Vec<Vec<i32>> = Vec::new();
    for _ in 0..positions_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let mut values = line.trim().split_whitespace();
        let x: i32 = values.next().unwrap().parse().unwrap();
        let y: i32 = values.next().unwrap().parse().unwrap();
        positions.push(vec![x, y]);
    }

    let result = max_moves(kx, ky, &positions, positions_size);
    println!("{}", result);

    Ok(())
}