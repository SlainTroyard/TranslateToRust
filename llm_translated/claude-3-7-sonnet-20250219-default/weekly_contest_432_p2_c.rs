use std::io::{self, BufRead};
use std::cmp;

// Problem: Weekly Contest 432 Problem 2
fn maximum_amount(coins: &Vec<Vec<i32>>) -> i32 {
    let row = coins.len() - 1;
    let col = coins[0].len() - 1;
    
    // Create a 3D DP array
    let mut dp = vec![vec![vec![0; 3]; col + 1]; row + 1];
    
    // Initialize the bottom-right corner
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = cmp::max(coins[row][col], 0);
    dp[row][col][2] = cmp::max(coins[row][col], 0);
    
    // Fill the bottom row from right to left
    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = cmp::max(dp[row][i + 1][0], dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = cmp::max(dp[row][i + 1][1], dp[row][i + 1][2] + coins[row][i]);
    }
    
    // Fill the rightmost column from bottom to top
    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = cmp::max(dp[i + 1][col][0], dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = cmp::max(dp[i + 1][col][1], dp[i + 1][col][2] + coins[i][col]);
    }
    
    // Fill the rest of the DP array
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = cmp::max(dp[j + 1][i][0], dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = cmp::max(
                cmp::max(dp[j][i + 1][0], dp[j][i + 1][1] + coins[j][i]),
                cmp::max(dp[j + 1][i][0], dp[j + 1][i][1] + coins[j][i])
            );
            dp[j][i][2] = cmp::max(
                cmp::max(dp[j][i + 1][1], dp[j][i + 1][2] + coins[j][i]),
                cmp::max(dp[j + 1][i][1], dp[j + 1][i][2] + coins[j][i])
            );
        }
    }
    
    dp[0][0][2]
}

fn main() {
    // Read n and m from the first line
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = dimensions[0];
    let m = dimensions[1];
    
    // Read the coins grid
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        coins.push(row);
    }
    
    // Calculate and print the result
    println!("{}", maximum_amount(&coins));
}