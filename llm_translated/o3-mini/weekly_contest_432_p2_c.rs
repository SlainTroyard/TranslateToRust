use std::cmp;
use std::error::Error;
use std::io::{self, BufRead};

/// This function implements the same DP algorithm from the original C code.
/// It takes a 2D vector `coins` and returns the maximum amount calculated.
fn maximum_amount(coins: &[Vec<i32>]) -> i32 {
    // Dimensions: n rows, m columns.
    let n = coins.len();
    let m = coins[0].len();

    // dp[i][j] is an array of three values.
    // We initialize dp as a 2D vector with each cell being [0, 0, 0].
    let mut dp = vec![vec![[0; 3]; m]; n];

    // Last cell indices.
    let row = n - 1;
    let col = m - 1;

    // Initialization for bottom-right cell.
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = cmp::max(coins[row][col], 0);
    dp[row][col][2] = cmp::max(coins[row][col], 0);

    // Fill the last row (only moving left).
    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = cmp::max(dp[row][i + 1][0], dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = cmp::max(dp[row][i + 1][1], dp[row][i + 1][2] + coins[row][i]);
    }

    // Fill the last column (only moving up).
    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = cmp::max(dp[i + 1][col][0], dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = cmp::max(dp[i + 1][col][1], dp[i + 1][col][2] + coins[i][col]);
    }

    // Fill the rest of the grid (cell by cell, from bottom to top and right to left).
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = cmp::max(dp[j + 1][i][0], dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = cmp::max(
                cmp::max(dp[j][i + 1][0], dp[j][i + 1][1] + coins[j][i]),
                cmp::max(dp[j + 1][i][0], dp[j + 1][i][1] + coins[j][i]),
            );
            dp[j][i][2] = cmp::max(
                cmp::max(dp[j][i + 1][1], dp[j][i + 1][2] + coins[j][i]),
                cmp::max(dp[j + 1][i][1], dp[j + 1][i][2] + coins[j][i]),
            );
        }
    }

    // The answer is stored at dp[0][0][2]
    dp[0][0][2]
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line and parse n and m.
    // The first line consists of two integers: n m
    let first_line = lines
        .next()
        .ok_or("Expected input for n and m")??;
    let mut dims = first_line.split_whitespace();
    let n: usize = dims
        .next()
        .ok_or("Expected n")?
        .parse()
        .map_err(|_| "Failed to parse n")?;
    let m: usize = dims
        .next()
        .ok_or("Expected m")?
        .parse()
        .map_err(|_| "Failed to parse m")?;

    // Read the next n lines, each containing m integers.
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines
            .next()
            .ok_or("Expected more input lines for coins")??;
        // Split the line by whitespace and parse each coin value.
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().map_err(|_| "Failed to parse coin value"))
            .collect::<Result<Vec<_>, _>>()?;
        // Ensure that each row has exactly m numbers.
        if row.len() != m {
            return Err(format!("Expected {} numbers in a row, got {}", m, row.len()).into());
        }
        coins.push(row);
    }

    // Compute the result using the DP algorithm.
    let result = maximum_amount(&coins);

    // Print the result (exact same stdout format as the original code).
    println!("{}", result);
    Ok(())
}