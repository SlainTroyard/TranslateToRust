use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::i32;
use std::cmp::{max, min};

// Solution struct to match the original class
struct Solution;

impl Solution {
    // Directions for the knight's movement (8 possible directions)
    const DIRS: [[i32; 2]; 8] = [[2, 1], [1, 2], [-1, 2], [-2, 1], [-2, -1], [-1, -2], [1, -2], [2, -1]];

    // Function to calculate the maximum number of moves
    fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        
        // Initialize the distance array with -1 (indicating not visited)
        let mut dis = vec![vec![vec![-1; 50]; 50]; n];

        // Calculate the number of moves required for the knight to reach each position
        for i in 0..n {
            let px = positions[i][0] as usize;
            let py = positions[i][1] as usize;
            dis[i][px][py] = 0;
            
            // Use a BFS to calculate the minimum steps from each soldier to all other positions
            let mut q = VecDeque::new();
            q.push_back((px, py));
            
            let mut step = 1;
            while !q.is_empty() {
                let size = q.len();
                for _ in 0..size {
                    if let Some((qx, qy)) = q.pop_front() {
                        for &[dx, dy] in &Self::DIRS {
                            let x = (qx as i32 + dx) as usize;
                            let y = (qy as i32 + dy) as usize;
                            
                            // Ensure the new position is within bounds and not yet visited
                            if x < 50 && y < 50 && dis[i][x][y] < 0 {
                                dis[i][x][y] = step;
                                q.push_back((x, y));
                            }
                        }
                    }
                }
                step += 1;
            }
        }

        // Add the knight's position to the list of positions
        positions.push(vec![kx, ky]);
        
        // Create memoization table
        let mut memo = vec![vec![-1; 1 << n]; n + 1];
        let u = (1 << n) - 1;

        // Define the recursive DFS function
        fn dfs(i: usize, mask: usize, n: usize, u: usize, positions: &Vec<Vec<i32>>, 
               dis: &Vec<Vec<Vec<i32>>>, memo: &mut Vec<Vec<i32>>) -> i32 {
            if mask == 0 {
                return 0; // No more soldiers to move
            }
            
            if memo[i][mask] != -1 {
                return memo[i][mask]; // Return cached result
            }
            
            let x = positions[i][0] as usize;
            let y = positions[i][1] as usize;
            
            let mut res = -1; // Initialize result
            
            // Check whose turn it is (Alice or Bob)
            if (u ^ mask).count_ones() % 2 == 0 { // Alice's move (maximize)
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        let next_result = dfs(j, mask ^ (1 << j), n, u, positions, dis, memo) + dis[j][x][y];
                        res = if res == -1 { next_result } else { max(res, next_result) };
                    }
                }
            } else { // Bob's move (minimize)
                res = i32::MAX;
                for j in 0..n {
                    if (mask >> j) & 1 == 1 {
                        res = min(res, dfs(j, mask ^ (1 << j), n, u, positions, dis, memo) + dis[j][x][y]);
                    }
                }
            }
            
            memo[i][mask] = res;
            res
        }

        dfs(n, u, n, u, &positions, &dis, &mut memo) // Start the DFS from the knight's position
    }
}

fn main() {
    // Set up input reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read knight's position and number of soldiers
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let kx = iter.next().unwrap();
    let ky = iter.next().unwrap();
    let n = iter.next().unwrap() as usize;
    
    // Read the positions of soldiers
    let mut positions = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let pos: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        positions.push(pos);
    }
    
    // Call the max_moves function and output the result
    let solution = Solution;
    let result = Solution::max_moves(kx, ky, positions);
    println!("{}", result);
}