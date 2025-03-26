use std::io::{self, BufRead};
use std::cmp::{max, min};

// Directions for the knight's movement (8 possible directions)
const DIRS: [[i32; 2]; 8] = [
    [2, 1], [1, 2], [-1, 2], [-2, 1], 
    [-2, -1], [-1, -2], [1, -2], [2, -1]
];

struct Solution;

impl Solution {
    // Function to calculate the maximum number of moves
    fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        
        // Initialize the distance array with -1 (equivalent to memset in C++)
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];
        
        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let px = positions[i][0] as usize;
            let py = positions[i][1] as usize;
            dis[i][px][py] = 0;
            
            let mut q = vec![(px, py)];
            let mut step = 1;
            
            // Perform a BFS to calculate the minimum steps from each soldier to all other positions
            while !q.is_empty() {
                let mut tmp = Vec::new();
                
                for (qx, qy) in q {
                    for &[dx, dy] in &DIRS {
                        let nx = qx as i32 + dx;
                        let ny = qy as i32 + dy;
                        
                        // Ensure the new position is within bounds and not yet visited
                        if nx >= 0 && nx < 50 && ny >= 0 && ny < 50 {
                            let x = nx as usize;
                            let y = ny as usize;
                            
                            if dis[i][x][y] < 0 {
                                dis[i][x][y] = step;
                                tmp.push((x, y));
                            }
                        }
                    }
                }
                
                q = tmp;
                step += 1;
            }
        }
        
        // Add the knight's position to the list of positions
        positions.push(vec![kx, ky]);
        // Initialize memoization table with -1 (results not computed yet)
        let mut memo = vec![vec![-1; 1 << n]; n + 1];
        let u = (1 << n) - 1;
        
        // Recursive DFS function for calculating optimal moves
        fn dfs(
            i: usize, 
            mask: usize, 
            n: usize, 
            u: usize, 
            positions: &[Vec<i32>], 
            dis: &[Vec<Vec<i32>>], 
            memo: &mut [Vec<i32>]
        ) -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            
            if memo[i][mask] != -1 {
                return memo[i][mask]; // Return cached result
            }
            
            let x = positions[i][0] as usize;
            let y = positions[i][1] as usize;
            
            // Check if it's Alice's move (equivalent to __builtin_parity)
            let is_alice_move = (u ^ mask).count_ones() % 2 == 0;
            
            let res = if is_alice_move {
                // Alice's move - maximize the result
                let mut best = -1;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        best = max(
                            best, 
                            dfs(j, mask ^ (1 << j), n, u, positions, dis, memo) + dis[j][x][y]
                        );
                    }
                }
                best
            } else {
                // Bob's move - minimize the result
                let mut best = i32::MAX;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        best = min(
                            best, 
                            dfs(j, mask ^ (1 << j), n, u, positions, dis, memo) + dis[j][x][y]
                        );
                    }
                }
                best
            };
            
            memo[i][mask] = res;
            res
        }
        
        // Start the DFS from the knight's position
        dfs(n, u, n, u, &positions, &dis, &mut memo)
    }
}

// Main function for I/O
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read knight's position (kx, ky) and number of soldiers
    let first_line = lines.next().unwrap()?;
    let values: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let kx = values[0];
    let ky = values[1];
    let n = values[2] as usize;
    
    // Read the positions of soldiers
    let mut positions = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let pos: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        
        positions.push(pos);
    }
    
    // Call the maxMoves function and output the result
    let result = Solution::max_moves(kx, ky, positions);
    println!("{}", result);
    
    Ok(())
}