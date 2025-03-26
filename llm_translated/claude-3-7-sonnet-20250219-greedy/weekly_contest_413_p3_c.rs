use std::cmp;
use std::io::{self, BufRead};

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
    
    // Create and initialize nums_raw array
    let mut nums_raw = vec![0; max_num as usize + 1];
    for i in 0..m {
        for j in 0..n {
            nums_raw[grid[i][j] as usize] |= 1 << i;
        }
    }
    
    // Create and initialize dp array
    let mut dp = vec![vec![i32::MIN; 1 << (m + 1)]; max_num as usize + 1];
    dp[0][0] = 0;
    
    for i in 1..=max_num as usize {
        for j in 0..(1 << m) {
            dp[i][j] = cmp::max(dp[i][j], dp[i-1][j]);
            
            // Try selecting element from row k
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
    
    // Read grid column sizes
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
        grid.push(row);
    }
    
    // Calculate and print the answer
    let ans = max_score(&grid, grid_size, &grid_col_size);
    println!("{}", ans);
}