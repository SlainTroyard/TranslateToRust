use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        let m = grid.len();
        let n = grid[0].len();
        
        // Create a 4D memoization array
        let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];
        
        // Define the DFS function
        fn dfs(
            i: usize, j: usize, k: usize, can_turn: bool, target: i32,
            grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Vec<Vec<i32>>>>, 
            dirs: &[[i32; 2]; 4]
        ) -> i32 {
            let m = grid.len();
            let n = grid[0].len();
            
            let ni = i as i32 + dirs[k][0];
            let nj = j as i32 + dirs[k][1];
            
            // Check if out of bounds or not matching target
            if ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32 || grid[ni as usize][nj as usize] != target {
                return 0;
            }
            
            let ni = ni as usize;
            let nj = nj as usize;
            
            // Check memoization
            let can_turn_idx = if can_turn { 1 } else { 0 };
            if memo[ni][nj][k][can_turn_idx] != 0 {
                return memo[ni][nj][k][can_turn_idx];
            }
            
            // Continue in the same direction
            let mut res = dfs(ni, nj, k, can_turn, 2 - target, grid, memo, dirs);
            
            // Try turning if allowed
            if can_turn {
                let nk = (k + 1) % 4;
                res = cmp::max(res, dfs(ni, nj, nk, false, 2 - target, grid, memo, dirs));
            }
            
            // Increment result and store in memo
            res += 1;
            memo[ni][nj][k][can_turn_idx] = res;
            
            res
        }
        
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        ans = cmp::max(ans, dfs(i, j, k, true, 2, &grid, &mut memo, &DIRS) + 1);
                    }
                }
            }
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read dimensions
    let dimensions = lines.next().unwrap().unwrap();
    let mut iter = dimensions.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Read grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..m {
            grid[i][j] = values[j];
        }
    }
    
    let sol = Solution;
    println!("{}", sol.len_of_v_diagonal(grid));
}