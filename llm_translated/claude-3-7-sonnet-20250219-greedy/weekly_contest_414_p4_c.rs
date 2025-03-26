use std::collections::VecDeque;
use std::io::{self, BufRead};

const N: usize = 50;
const INF: i32 = i32::MAX;

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
            
            while !queue.is_empty() && !found {
                let size = queue.len();
                for _ in 0..size {
                    let current = queue.pop_front().unwrap();
                    
                    if current.x == sol.pos[j].x && current.y == sol.pos[j].y {
                        sol.dist[i][j] = steps;
                        found = true;
                        break;
                    }
                    
                    for (dx, dy) in directions.iter() {
                        let nx = current.x + dx;
                        let ny = current.y + dy;
                        
                        if is_valid(nx, ny) && !seen[nx as usize][ny as usize] {
                            queue.push_back(Position { x: nx, y: ny });
                            seen[nx as usize][ny as usize] = true;
                        }
                    }
                }
                
                if !found {
                    steps += 1;
                }
            }
        }
    }
}

fn dfs(sol: &Solution, i: usize, m: u64, turn: usize, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    if m == (1u64 << sol.n) - 1 {
        return 0;
    }
    
    if memo[i][m as usize][turn] != -1 {
        return memo[i][m as usize][turn];
    }
    
    let mut ans = if turn == 0 { 0 } else { INF };
    
    for k in 0..sol.n {
        if (m & (1u64 << k)) == 0 {
            let next_m = m | (1u64 << k);
            let result = sol.dist[i][k] + dfs(sol, k, next_m, 1 - turn, memo);
            
            if turn == 0 {
                ans = ans.max(result);
            } else {
                ans = ans.min(result);
            }
        }
    }
    
    memo[i][m as usize][turn] = ans;
    return ans;
}

fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let positions_size = positions.len();
    
    let mut sol = Solution {
        pos: vec![Position { x: 0, y: 0 }; positions_size + 1],
        dist: vec![vec![0; positions_size]; positions_size + 1],
        n: positions_size,
    };
    
    for i in 0..positions_size {
        sol.pos[i] = Position { x: positions[i][0], y: positions[i][1] };
    }
    sol.pos[positions_size] = Position { x: kx, y: ky };
    
    calculate_distances(&mut sol);
    
    // Create memo array
    let max_state = 1u64 << positions_size;
    let mut memo = vec![vec![vec![-1; 2]; max_state as usize]; N + 1];
    
    dfs(&sol, positions_size, 0, 0, &mut memo)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read knight position
    let knight_pos: Vec<i32> = lines.next().unwrap()?
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let kx = knight_pos[0];
    let ky = knight_pos[1];
    
    // Read positions size
    let positions_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read positions
    let mut positions = Vec::with_capacity(positions_size);
    for _ in 0..positions_size {
        let pos: Vec<i32> = lines.next().unwrap()?
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        positions.push(pos);
    }
    
    let result = max_moves(kx, ky, positions);
    println!("{}", result);
    
    Ok(())
}