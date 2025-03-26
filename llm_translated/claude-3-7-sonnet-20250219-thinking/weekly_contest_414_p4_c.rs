use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::cmp::{max, min};

const N: usize = 50;
const INF: i32 = i32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
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
            if i == j { continue; }
            
            let mut queue = VecDeque::new();
            queue.push_back(sol.pos[i]);
            
            let mut seen = vec![vec![false; N]; N];
            seen[sol.pos[i].x][sol.pos[i].y] = true;
            
            let mut steps = 0;
            
            'bfs: loop {
                if queue.is_empty() { break; }
                
                let size = queue.len();
                for _ in 0..size {
                    let current = queue.pop_front().unwrap();
                    
                    if current == sol.pos[j] {
                        sol.dist[i][j] = steps;
                        break 'bfs;
                    }
                    
                    for &(dx, dy) in &directions {
                        let nx = current.x as i32 + dx;
                        let ny = current.y as i32 + dy;
                        
                        if is_valid(nx, ny) {
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

fn dfs(sol: &Solution, i: usize, m: u64, turn: usize, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
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

fn max_moves(kx: usize, ky: usize, positions: &Vec<Vec<i32>>, positions_size: usize) -> i32 {
    let mut sol = Solution {
        pos: vec![Position { x: 0, y: 0 }; positions_size + 1],
        dist: vec![vec![0; positions_size]; positions_size + 1],
        n: positions_size,
    };
    
    for i in 0..positions_size {
        sol.pos[i] = Position {
            x: positions[i][0] as usize,
            y: positions[i][1] as usize,
        };
    }
    
    sol.pos[positions_size] = Position { x: kx, y: ky };
    
    calculate_distances(&mut sol);
    
    // Create memoization table
    let max_states = 1 << positions_size;
    let mut memo = vec![vec![vec![-1; 2]; max_states]; positions_size + 1];
    
    dfs(&sol, positions_size, 0, 0, &mut memo)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse king's position
    let first_line = lines.next().unwrap()?;
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let kx = parts[0];
    let ky = parts[1];
    
    // Parse number of positions
    let positions_size: usize = lines.next().unwrap()?.parse().unwrap();
    
    // Parse positions
    let mut positions = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        let line = lines.next().unwrap()?;
        let coords: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        positions.push(coords);
    }
    
    let result = max_moves(kx, ky, &positions, positions_size);
    println!("{}", result);
    
    Ok(())
}