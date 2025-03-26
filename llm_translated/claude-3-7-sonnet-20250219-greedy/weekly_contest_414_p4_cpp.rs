use std::collections::VecDeque;
use std::io::{self, BufRead};

// Directions for the knight's movement (8 possible directions)
const DIRS: [[i32; 2]; 8] = [
    [2, 1], [1, 2], [-1, 2], [-2, 1], [-2, -1], [-1, -2], [1, -2], [2, -1]
];

struct Solution;

impl Solution {
    // Function to calculate the maximum number of moves
    pub fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        // Create a 3D array to store distances
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let px = positions[i][0] as usize;
            let py = positions[i][1] as usize;
            dis[i][px][py] = 0;
            
            let mut q = VecDeque::new();
            q.push_back((px, py));
            
            // Perform a BFS to calculate the minimum steps from each soldier to all other positions
            let mut step = 1;
            while !q.is_empty() {
                let size = q.len();
                for _ in 0..size {
                    let (qx, qy) = q.pop_front().unwrap();
                    for &[dx, dy] in DIRS.iter() {
                        let x = (qx as i32 + dx) as usize;
                        let y = (qy as i32 + dy) as usize;
                        // Ensure the new position is within bounds and not yet visited
                        if x < 50 && y < 50 && dis[i][x][y] < 0 {
                            dis[i][x][y] = step;
                            q.push_back((x, y));
                        }
                    }
                }
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        positions.push(vec![kx, ky]);
        let mut memo = vec![vec![-1; 1 << n]; n + 1]; // -1 indicates that the result is not yet computed
        let u = (1 << n) - 1;

        // Define a recursive function for DFS
        fn dfs(
            i: usize, 
            mask: usize, 
            positions: &Vec<Vec<i32>>, 
            dis: &Vec<Vec<Vec<i32>>>, 
            memo: &mut Vec<Vec<i32>>, 
            u: usize, 
            n: usize
        ) -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            
            if memo[i][mask] != -1 {
                return memo[i][mask]; // Return memoized result
            }
            
            let x = positions[i][0] as usize;
            let y = positions[i][1] as usize;
            
            // Calculate parity similar to __builtin_parity
            let parity = (u ^ mask).count_ones() % 2 == 0;
            
            if parity { // Alice's move
                let mut res = -1;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        let next_result = dfs(j, mask ^ (1 << j), positions, dis, memo, u, n) + dis[j][x][y];
                        res = res.max(next_result);
                    }
                }
                memo[i][mask] = res;
            } else { // Bob's move
                let mut res = i32::MAX;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        let next_result = dfs(j, mask ^ (1 << j), positions, dis, memo, u, n) + dis[j][x][y];
                        res = res.min(next_result);
                    }
                }
                memo[i][mask] = res;
            }
            
            memo[i][mask]
        }

        // Start the DFS from the knight's position
        dfs(n, u, &positions, &dis, &mut memo, u, n)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read knight's position (kx, ky) and number of soldiers
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let kx: i32 = iter.next().unwrap().parse().unwrap();
    let ky: i32 = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    
    // Read the positions of soldiers
    let mut positions = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        positions.push(vec![x, y]);
    }
    
    // Call the max_moves function and output the result
    let result = Solution::max_moves(kx, ky, positions);
    println!("{}", result);
}