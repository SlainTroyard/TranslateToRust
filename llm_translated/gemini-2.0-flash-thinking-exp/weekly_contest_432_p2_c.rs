use std::io;
use std::cmp::max;

fn maximum_amount(coins: &Vec<Vec<i32>>) -> i32 {
    let row_size = coins.len();
    let col_size = if row_size > 0 { coins[0].len() } else { 0 };

    if row_size == 0 || col_size == 0 {
        return 0;
    }

    let row = row_size - 1;
    let col = col_size - 1;
    let mut dp = vec![vec![vec![0; 3]; col + 1]; row + 1];

    // Base case initialization for dp table, same as C code logic
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = max(coins[row][col], 0);
    dp[row][col][2] = max(coins[row][col], 0);

    // Fill dp table for the last row (bottom row), same as C code logic
    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = max(dp[row][i + 1][0], dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = max(dp[row][i + 1][1], dp[row][i + 1][2] + coins[row][i]);
    }

    // Fill dp table for the last column (rightmost column), same as C code logic
    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = max(dp[i + 1][col][0], dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = max(dp[i + 1][col][1], dp[i + 1][col][2] + coins[i][col]);
    }

    // Fill the rest of the dp table in reverse order, same as C code logic
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = max(dp[j + 1][i][0], dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = max(
                max(dp[j][i + 1][0], dp[j][i + 1][1] + coins[j][i]),
                max(dp[j + 1][i][0], dp[j + 1][i][1] + coins[j][i]),
            );
            dp[j][i][2] = max(
                max(dp[j][i + 1][1], dp[j][i + 1][2] + coins[j][i]),
                max(dp[j + 1][i][1], dp[j + 1][i][2] + coins[j][i]),
            );
        }
    }

    dp[0][0][2] // Return the final result, same as C code logic
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");

    let mut coins: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row_coins: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse coin value"))
            .collect();
        coins.push(row_coins);
    }

    println!("{}", maximum_amount(&coins));
}