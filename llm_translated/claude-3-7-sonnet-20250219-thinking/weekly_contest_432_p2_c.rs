use std::io::{self, BufRead};
use std::cmp::max;

fn maximum_amount(coins: &[Vec<i32>]) -> i32 {
    let row = coins.len() - 1;
    let col = coins[0].len() - 1;
    
    // Create 3D DP array initialized to 0
    let mut dp = vec![vec![vec![0; 3]; col + 1]; row + 1];
    
    // Base case: initialize the bottom-right cell
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = max(coins[row][col], 0);
    dp[row][col][2] = max(coins[row][col], 0);
    
    // Fill in the last row
    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = max(dp[row][i + 1][0], dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = max(dp[row][i + 1][1], dp[row][i + 1][2] + coins[row][i]);
    }
    
    // Fill in the last column
    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = max(dp[i + 1][col][0], dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = max(dp[i + 1][col][1], dp[i + 1][col][2] + coins[i][col]);
    }
    
    // Fill in the rest of the DP array
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = max(dp[j + 1][i][0], dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = max(
                max(dp[j][i + 1][0], dp[j][i + 1][1] + coins[j][i]),
                max(dp[j + 1][i][0], dp[j + 1][i][1] + coins[j][i])
            );
            dp[j][i][2] = max(
                max(dp[j][i + 1][1], dp[j][i + 1][2] + coins[j][i]),
                max(dp[j + 1][i][1], dp[j + 1][i][2] + coins[j][i])
            );
        }
    }
    
    dp[0][0][2]
}

fn main() {
    // Read n and m from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");
    
    // Read the coins grid
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::with_capacity(m);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse each value in the row
        for val in input.split_whitespace() {
            let coin: i32 = val.parse().expect("Failed to parse coin value");
            row.push(coin);
        }
        
        coins.push(row);
    }
    
    // Call the function and print the result
    println!("{}", maximum_amount(&coins));
}