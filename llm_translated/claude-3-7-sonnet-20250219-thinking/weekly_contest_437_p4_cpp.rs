use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
        const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, -1], [-1, 1]];
        let m = grid.len();
        let n = grid[0].len();
        
        // Initialize memoization table
        let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];
        
        // Recursive DFS function with closure
        fn dfs(
            i: i32, j: i32, k: usize, can_turn: bool, target: i32,
            grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
            m: usize, n: usize, dirs: &[[i32; 2]; 4]
        ) -> i32 {
            if i < 0 || i >= m as i32 || j < 0 || j >= n as i32 {
                return 0;
            }
            
            let ni = i + dirs[k][0];
            let nj = j + dirs[k][1];
            
            if ni < 0 || nj < 0 || ni >= m as i32 || nj >= n as i32 || 
               grid[ni as usize][nj as usize] != target {
                return 0;
            }
            
            let ni_usize = ni as usize;
            let nj_usize = nj as usize;
            let can_turn_idx = if can_turn { 1 } else { 0 };
            
            if memo[ni_usize][nj_usize][k][can_turn_idx] != 0 {
                return memo[ni_usize][nj_usize][k][can_turn_idx];
            }
            
            let mut res = dfs(ni, nj, k, can_turn, 2 - target, grid, memo, m, n, dirs);
            
            if can_turn {
                let nk = (k + 1) % 4;
                res = max(res, dfs(ni, nj, nk, false, 2 - target, grid, memo, m, n, dirs));
            }
            
            memo[ni_usize][nj_usize][k][can_turn_idx] = res + 1;
            res + 1
        }
        
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    for k in 0..4 {
                        ans = max(ans, dfs(i as i32, j as i32, k, true, 2, grid, &mut memo, m, n, &DIRS) + 1);
                    }
                }
            }
        }
        
        ans
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse dimensions
    let dimensions = lines.next().unwrap().unwrap();
    let mut iter = dimensions.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Parse grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let row = lines.next().unwrap().unwrap();
        let values: Vec<i32> = row.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..m {
            grid[i][j] = values[j];
        }
    }
    
    // Solve and output
    let solution = Solution::len_of_v_diagonal(&grid);
    println!("{}", solution);
}