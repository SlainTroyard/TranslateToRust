use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut pos: HashMap<i32, i32> = HashMap::new();
        let m = grid.len();
        
        // Build a map of each number to a bitmask representing which rows it appears in
        for i in 0..m {
            for &x in &grid[i] {
                let entry = pos.entry(x).or_insert(0);
                *entry |= 1 << i;
            }
        }

        // Collect all unique numbers
        let all_nums: Vec<i32> = pos.keys().cloned().collect();
        let n = all_nums.len();
        
        // Initialize memoization table
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; 1 << m]; n];
        
        // Define DFS function using closure
        fn dfs(
            i: i32, 
            j: i32, 
            all_nums: &Vec<i32>, 
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
            
            // Skip current number
            let mut res = dfs(i - 1, j, all_nums, pos, memo);
            
            let x = all_nums[i_usize];
            let mut t = *pos.get(&x).unwrap();
            
            // Try to use current number in each possible row
            while t != 0 {
                let lb = t & -t; // Lowest bit
                if (j & lb) == 0 {
                    res = res.max(dfs(i - 1, j | lb, all_nums, pos, memo) + x);
                }
                t ^= lb; // Remove the lowest bit
            }
            
            memo[i_usize][j_usize] = res;
            return res;
        }
        
        dfs(n as i32 - 1, 0, &all_nums, &pos, &mut memo)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid size
    let grid_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        // Read column size for this row
        let grid_col_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        // Read row values
        let row: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        grid.push(row);
    }
    
    let sol = Solution;
    println!("{}", sol.max_score(grid));
}