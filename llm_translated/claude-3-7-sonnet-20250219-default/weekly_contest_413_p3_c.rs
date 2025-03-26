use std::io::{self, BufRead};
use std::cmp;

fn max_score(grid: &Vec<Vec<i32>>, grid_size: usize, grid_col_size: &Vec<usize>) -> i32 {
    let mut ans = 0;
    let mut max_num = 0;
    let m = grid_size;
    let n = grid_col_size[0];
    
    // Find the maximum value in the grid
    for i in 0..m {
        for j in 0..n {
            max_num = cmp::max(max_num, grid[i][j]);
        }
    }
    
    // Initialize nums_raw array to track which rows contain each value
    let mut nums_raw = vec![0; (max_num + 1) as usize];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }
    
    // Initialize dp array for dynamic programming
    let max_mask = 1 << m;
    let mut dp = vec![vec![i32::MIN; max_mask]; (max_num + 1) as usize];
    dp[0][0] = 0;
    
    for i in 1..=max_num as usize {
        for j in 0..max_mask {
            // Don't select any element with value i
            dp[i][j] = cmp::max(dp[i][j], dp[i-1][j]);
            
            // Try selecting an element with value i from each row
            for k in 0..m {
                if ((nums_raw[i] >> k) & 1) != 0 && ((j >> k) & 1) != 0 {
                    dp[i][j] = cmp::max(dp[i][j], dp[i-1][j ^ (1 << k)] + i as i32);
                    ans = cmp::max(ans, dp[i][j]);
                }
            }
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read grid size
    let grid_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read column sizes
    let grid_col_size: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Read grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for i in 0..grid_size {
        let row: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        // Ensure we read exactly the specified number of columns
        assert_eq!(row.len(), grid_col_size[i]);
        grid.push(row);
    }
    
    // Calculate and print the result
    let ans = max_score(&grid, grid_size, &grid_col_size);
    println!("{}", ans);
}