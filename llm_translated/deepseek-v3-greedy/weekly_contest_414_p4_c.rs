use std::collections::VecDeque;
use std::cmp::{max, min};
use std::mem;

const N: usize = 50;
const INF: i32 = i32::MAX;

#[derive(Clone, Copy)]
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

    fn dequeue(&mut self) -> Position {
        self.queue.pop_front().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32
}

impl Solution {
    fn calculate_distances(&mut self) {
        let directions: [[i32; 2]; 8] = [
            [-2, -1], [-2, 1], [-1, -2], [-1, 2],
            [1, -2], [1, 2], [2, -1], [2, 1],
        ];

        for i in 0..=self.n {
            for j in 0..self.n {
                if i == j {
                    continue;
                }
                let mut queue = Queue::new();
                queue.enqueue(self.pos[i]);
                let mut seen = [[false; N]; N];
                seen[self.pos[i].x as usize][self.pos[i].y as usize] = true;
                let mut steps = 0;

                while !queue.is_empty() {
                    let size = queue.queue.len();
                    for _ in 0..size {
                        let current = queue.dequeue();
                        if current.x == self.pos[j].x && current.y == self.pos[j].y {
                            self.dist[i][j] = steps;
                            break;
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
                    steps += 1;
                }
            }
        }
    }
}

fn dfs(sol: &Solution, i: usize, m: u64, turn: usize, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if memo[i][m as usize][turn] != -1 {
        return memo[i][m as usize][turn];
    }

    let mut ans = if turn == 0 { 0 } else { INF };
    for k in 0..sol.n {
        if (m & (1 << k)) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
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

fn max_moves(kx: i32, ky: i32, positions: &[Vec<i32>]) -> i32 {
    let mut sol = Solution {
        pos: [Position { x: 0, y: 0 }; N + 1],
        dist: [[0; N]; N + 1],
        n: positions.len(),
    };

    for i in 0..positions.len() {
        sol.pos[i] = Position {
            x: positions[i][0],
            y: positions[i][1],
        };
    }
    sol.pos[positions.len()] = Position { x: kx, y: ky };

    sol.calculate_distances();

    let mut memo = vec![vec![vec![-1; 2]; 1 << positions.len()]; N + 1];
    let result = dfs(&sol, sol.n, 0, 0, &mut memo);

    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let kx: i32 = iter.next().unwrap().parse().unwrap();
    let ky: i32 = iter.next().unwrap().parse().unwrap();

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let positions_size: usize = input.trim().parse().unwrap();

    let mut positions = Vec::new();
    for _ in 0..positions_size {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        positions.push(vec![x, y]);
    }

    let result = max_moves(kx, ky, &positions);
    println!("{}", result);
}