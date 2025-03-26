use std::io::{self, BufRead};
use std::cmp::{max, min};

const N: usize = 50;
const INF: i32 = std::i32::MAX;
const MAX_STATE: u64 = 1 << N;

#[derive(Copy, Clone)]
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
    queue: [Position; 2500],
    front: usize,
    rear: usize,
}

impl Queue {
    fn new() -> Self {
        Queue {
            queue: [Position { x: 0, y: 0 }; 2500],
            front: 0,
            rear: 0,
        }
    }

    fn enqueue(&mut self, p: Position) {
        self.queue[self.rear] = p;
        self.rear += 1;
    }

    fn dequeue(&mut self) -> Position {
        let p = self.queue[self.front];
        self.front += 1;
        p
    }

    fn is_empty(&self) -> bool {
        self.front == self.rear
    }
}

fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32
}

fn calculate_distances(sol: &mut Solution) {
    let directions = [
        [-2, -1], [-2, 1], [-1, -2], [-1, 2],
        [1, -2], [1, 2], [2, -1], [2, 1]
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

            while !queue.is_empty() {
                let size = queue.rear - queue.front;
                for _ in 0..size {
                    let current = queue.dequeue();
                    if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                        sol.dist[i][j] = steps;
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
                if sol.dist[i][j] != 0 {
                    break;
                }
                steps += 1;
            }
        }
    }
}

fn dfs(sol: &Solution, i: usize, m: u64, turn: i32, memo: &mut [[[i32; 2]; MAX_STATE as usize]; N + 1]) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if memo[i][m as usize][turn as usize] != -1 {
        return memo[i][m as usize][turn as usize];
    }

    let mut ans = if turn == 0 { 0 } else { INF };
    for k in 0..sol.n {
        if m & (1 << k) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                ans = max(result, ans);
            } else {
                ans = min(result, ans);
            }
        }
    }
    memo[i][m as usize][turn as usize] = ans;
    ans
}

fn max_moves(kx: i32, ky: i32, positions: &Vec<Vec<i32>>) -> i32 {
    let mut sol = Solution {
        pos: [Position { x: 0, y: 0 }; N + 1],
        dist: [[0; N]; N + 1],
        n: positions.len(),
    };
    for i in 0..positions.len() {
        sol.pos[i] = Position { x: positions[i][0], y: positions[i][1] };
    }
    sol.pos[positions.len()] = Position { x: kx, y: ky };

    calculate_distances(&mut sol);

    let mut memo = [[[-1; 2]; MAX_STATE as usize]; N + 1];
    dfs(&sol, sol.n, 0, 0, &mut memo)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let kx: i32 = iter.next().unwrap().parse().unwrap();
    let ky: i32 = iter.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap()?;
    let positions_size: usize = second_line.parse().unwrap();

    let mut positions = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        positions.push(vec![x, y]);
    }

    let result = max_moves(kx, ky, &positions);
    println!("{}", result);

    Ok(())
}