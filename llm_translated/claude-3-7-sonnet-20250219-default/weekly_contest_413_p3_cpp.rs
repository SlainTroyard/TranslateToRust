use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut pos: HashMap<i32, i32> = HashMap::new();
        let m = grid.len();
        
        // Create a bitmap for each number indicating which rows it appears in
        for i in 0..m {
            for &x in &grid[i] {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        // Collect all unique numbers
        let all_nums: Vec<i32> = pos.keys().cloned().collect();
        let n = all_nums.len();
        
        // Initialize memoization table
        let mut memo = vec![vec![-1; 1 << m]; n];
        
        // Define DFS function
        fn dfs(
            i: i32, 
            j: i32, 
            all_nums: &[i32], 
            pos: &HashMap<i32, i32>, 
            memo: &mut Vec<Vec<i32>>
        ) -> i32 {
            if i < 0 {
                return 0;
            }
            
            let i_usize = i as usize;
            let j_usize = j as usize;
            
            if memo[i_usize][j_usize] != -1 {
                return memo[i_usize][j_usize];
            }
            
            // Skip the current number
            let mut res = dfs(i - 1, j, all_nums, pos, memo);
            
            let x = all_nums[i_usize];
            let mut t = pos[&x];
            
            // Try to use the current number in each of its possible rows
            while t != 0 {
                let lb = t & -t; // Lowest bit
                if (j & lb) == 0 { // If this row hasn't been used yet
                    res = res.max(dfs(i - 1, j | lb, all_nums, pos, memo) + x);
                }
                t ^= lb; // Remove the lowest bit
            }
            
            memo[i_usize][j_usize] = res;
            return res;
        }
        
        dfs((n as i32) - 1, 0, &all_nums, &pos, &mut memo)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();
    
    // Read grid size
    let grid_size: usize = input_lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut grid = Vec::with_capacity(grid_size);
    
    // Read each row of the grid
    for _ in 0..grid_size {
        // Read the number of elements in this row
        let grid_col_size: usize = input_lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        // Read the row values
        let row_str = input_lines.next().unwrap().unwrap();
        let row: Vec<i32> = row_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        grid.push(row);
    }
    
    let solution = Solution::max_score(grid);
    println!("{}", solution);
}