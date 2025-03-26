use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::process::exit;

const INF: i32 = i32::MAX;
const N: usize = 50;
const MAX_STATE: i64 = 1 << N;

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
            let mut queue: VecDeque<Position> = VecDeque::new();
            queue.push_back(sol.pos[i]);
            let mut seen = [[false; N]; N];
            seen[sol.pos[i].x as usize][sol.pos[i].y as usize] = true;
            let mut steps = 0;

            'outer: while !queue.is_empty() {
                let size = queue.len();
                for _ in 0..size {
                    if let Some(current) = queue.pop_front() {
                        if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                            sol.dist[i][j] = steps;
                            break 'outer;
                        }
                        for d in 0..8 {
                            let nx = current.x + directions[d][0];
                            let ny = current.y + directions[d][1];
                            if is_valid(nx, ny) && !seen[nx as usize][ny as usize] {
                                queue.push_back(Position { x: nx, y: ny });
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

fn dfs(sol: &Solution, i: usize, m: i64, turn: i32, memo: &mut Vec<Vec<Vec<Option<i32>>>>) -> i32 {
    if m == (1 << sol.n) - 1 {
        return 0;
    }
    if let Some(val) = memo[i][m as usize][turn as usize] {
        return val;
    }

    let mut ans = if turn == 0 { 0 } else { INF };
    for k in 0..sol.n {
        if (m & (1 << k)) == 0 {
            let next_m = m | (1 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
            if turn == 0 {
                ans = ans.max(result);
            } else {
                ans = ans.min(result);
            }
        }
    }
    memo[i][m as usize][turn as usize] = Some(ans);
    ans
}

fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let positions_size = positions.len();
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

    calculate_distances(&mut sol);

    let mut memo: Vec<Vec<Vec<Option<i32>>>> = vec![vec![vec![None; 2]; MAX_STATE as usize]; N + 1];
    dfs(&sol, sol.n, 0, 0, &mut memo)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let kx: i32 = parts.next().unwrap().parse().unwrap();
    let ky: i32 = parts.next().unwrap().parse().unwrap();

    let positions_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut positions: Vec<Vec<i32>> = Vec::new();
    for _ in 0..positions_size {
        let line = lines.next().unwrap().unwrap();
        let parts_pos: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        positions.push(parts_pos);
    }

    let result = max_moves(kx, ky, positions);
    println!("{}", result);
}