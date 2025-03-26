use std::collections::VecDeque;
use std::io::{self, BufRead};

const INF: i32 = i32::MAX;
const N: usize = 50;
const MAX_STATE: u64 = 1 << N; // Use u64 for large bitmask

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

struct Solution {
    pos: Vec<Position>,
    dist: Vec<Vec<i32>>,
    n: usize,
}

fn is_valid(x: i32, y: i32) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32
}

fn calculate_distances(sol: &mut Solution) {
    let directions = [
        (-2, -1), (-2, 1), (-1, -2), (-1, 2),
        (1, -2), (1, 2), (2, -1), (2, 1)
    ];
    
    for i in 0..=sol.n {
        for j in 0..sol.n {
            if i == j {
                continue;
            }
            
            let mut queue = VecDeque::new();
            queue.push_back(sol.pos[i]);
            
            let mut seen = vec![vec![false; N]; N];
            seen[sol.pos[i].x as usize][sol.pos[i].y as usize] = true;
            
            let mut steps = 0;
            let mut found = false;
            
            'bfs: while !queue.is_empty() {
                let size = queue.len();
                for _ in 0..size {
                    let current = queue.pop_front().unwrap();
                    
                    if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                        sol.dist[i][j] = steps;
                        found = true;
                        break 'bfs;
                    }
                    
                    for (dx, dy) in directions {
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
            
            if !found {
                // Ensure dist is set even if not found (though this shouldn't happen)
                sol.dist[i][j] = INF;
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
                ans = ans.max(result);
            } else {
                ans = ans.min(result);
            }
        }
    }
    
    memo[i][m as usize][turn] = ans;
    ans
}

fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let positions_size = positions.len();
    
    let mut sol = Solution {
        pos: Vec::with_capacity(positions_size + 1),
        dist: vec![vec![0; positions_size]; positions_size + 1],
        n: positions_size,
    };
    
    for i in 0..positions_size {
        sol.pos.push(Position {
            x: positions[i][0],
            y: positions[i][1],
        });
    }
    
    sol.pos.push(Position { x: kx, y: ky });
    
    calculate_distances(&mut sol);
    
    // Initialize memo array
    let max_state = 1 << positions_size;
    let mut memo = vec![vec![vec![-1; 2]; max_state]; N + 1];
    
    dfs(&sol, positions_size, 0, 0, &mut memo)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse knight position
    let line = lines.next().unwrap()?;
    let mut parts = line.split_whitespace();
    let kx: i32 = parts.next().unwrap().parse().unwrap();
    let ky: i32 = parts.next().unwrap().parse().unwrap();
    
    // Parse positions size
    let positions_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Parse positions
    let mut positions = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        let line = lines.next().unwrap()?;
        let mut parts = line.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        positions.push(vec![x, y]);
    }
    
    let result = max_moves(kx, ky, positions);
    println!("{}", result);
    
    Ok(())
}