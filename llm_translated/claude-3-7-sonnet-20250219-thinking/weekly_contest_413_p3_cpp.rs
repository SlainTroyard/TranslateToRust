use std::collections::HashMap;
use std::io::{self, BufRead};
use std::cmp;

struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut pos: HashMap<i32, i32> = HashMap::new();
        let m = grid.len();
        
        // Map each number to a bitmask of rows it appears in
        for i in 0..m {
            for &x in &grid[i] {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        // Collect all unique numbers
        let mut all_nums: Vec<i32> = Vec::new();
        for (&x, _) in &pos {
            all_nums.push(x);
        }

        let n = all_nums.len();
        let mut memo = vec![vec![-1; 1 << m]; n];
        
        // Recursive function for dynamic programming
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
            
            // Option 1: Skip the current number
            let mut res = dfs(i - 1, j, all_nums, pos, memo);
            
            // Option 2: Use the current number in one of its available rows
            let x = all_nums[i_usize];
            let mut t = *pos.get(&x).unwrap();
            
            while t != 0 {
                let lb = t & -t; // Get least significant bit
                if (j & lb) == 0 { // If this row is not used yet
                    res = cmp::max(res, dfs(i - 1, j | lb, all_nums, pos, memo) + x);
                }
                t ^= lb; // Remove the least significant bit
            }
            
            memo[i_usize][j_usize] = res;
            res
        }
        
        dfs(n as i32 - 1, 0, &all_nums, &pos, &mut memo)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid size
    let grid_size: usize = lines.next()
        .expect("Failed to read input")?
        .trim()
        .parse()
        .expect("Failed to parse grid size");
    
    let mut grid = Vec::with_capacity(grid_size);
    
    // Read each row
    for _ in 0..grid_size {
        // Read column size for this row
        let grid_col_size: usize = lines.next()
            .expect("Failed to read input")?
            .trim()
            .parse()
            .expect("Failed to parse grid column size");
        
        // Read row elements
        let row: Vec<i32> = lines.next()
            .expect("Failed to read input")?
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse grid element"))
            .take(grid_col_size)
            .collect();
        
        grid.push(row);
    }
    
    // Calculate and output the result
    println!("{}", Solution::max_score(grid));
    
    Ok(())
}