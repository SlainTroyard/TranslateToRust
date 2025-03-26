use std::io::{self, BufRead, Write};
use std::collections::VecDeque;

const INF: i32 = i32::MAX;
const N: usize = 50;
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

fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32
}

fn calculate_distances(sol: &mut Solution) {
    let directions = [
        (-2, -1), (-2, 1), (-1, -2), (-1, 2),
        (1, -2), (1, 2), (2, -1), (2, 1),
    ];

    for i in 0..=sol.n {
        for j in 0..sol.n {
            if i == j { continue; }
            let mut queue = VecDeque::new();
            queue.push_back(sol.pos[i]);
            let mut seen = vec![vec![false; N]; N];
            seen[sol.pos[i].x as usize][sol.pos[i].y as usize] = true;
            let mut steps = 0;

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let current = queue.pop_front().unwrap();
                    if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                        sol.dist[i][j] = steps;
                        continue 'outer;
                    }
                    for (dx, dy) in &directions {
                        let nx = current.x + dx;
                        let ny = current.y + dy;
                        if is_valid(nx, ny) && !seen[nx as usize][ny as usize] {
                            queue.push_back(Position { x: nx, y: ny });
                            seen[nx as usize][ny as usize] = true;
                        }
                    }
                }
                steps += 1;
            }
            'outer: {}
        }
    }
}

fn dfs(sol: &Solution, i: usize, m: u64, turn: usize, memo: &mut [[[i32; 2]; 1 << N]; N + 1]) -> i32 {
    if m == (1 << sol.n) - 1 { return 0; }
    if memo[i][m as usize][turn] != -1 { return memo[i][m as usize][turn]; }

    let mut ans = if turn == 0 { 0 } else { INF };
    for k in 0..sol.n {
        if m & (1 << k) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                ans = ans.max(result);
            } else {
                ans = ans.min(result);
            }
        }
    }
    memo[i][m as usize][turn] = ans;
    ans
}

fn max_moves(kx: i32, ky: i32, positions: &[(i32, i32)], positions_size: usize) -> i32 {
    let mut sol = Solution {
        pos: [Position { x: 0, y: 0 }; N + 1],
        dist: [[0; N]; N + 1],
        n: positions_size,
    };

    for i in 0..positions_size {
        sol.pos[i] = Position { x: positions[i].0, y: positions[i].1 };
    }
    sol.pos[positions_size] = Position { x: kx, y: ky };

    let mut memo = [[[0; 2]; 1 << N]; N + 1];
    for i in 0..=N {
        for j in 0..(1 << N) {
            memo[i][j][0] = -1;
            memo[i][j][1] = -1;
        }
    }

    calculate_distances(&mut sol);
    dfs(&sol, sol.n, 0, 0, &mut memo)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let kx = iter.next().unwrap().parse::<i32>().unwrap();
    let ky = iter.next().unwrap().parse::<i32>().unwrap();

    input.clear();
    stdin_lock.read_line(&mut input).unwrap();
    let positions_size = input.trim().parse::<usize>().unwrap();

    let mut positions = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        input.clear();
        stdin_lock.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        positions.push((x, y));
    }

    let result = max_moves(kx, ky, &positions, positions_size);
    writeln!(stdout_lock, "{}", result).unwrap();
}