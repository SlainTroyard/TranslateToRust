use std::io::{self, BufRead};
use std::error::Error;

/// Translates the C function `maximumAmount(int** coins, int coinsSize, int* coinsColSize)`
/// to an idiomatic Rust function computing the same result.
///
/// # Arguments
/// * `coins` - A 2D vector representing the grid of coin values.
/// * `coins_size` - The number of rows in the grid.
/// * `coins_col_size` - A vector carrying the number of columns per row.
///
/// # Returns
/// * The computed maximum amount as per the DP logic in the C code.
fn maximum_amount(coins: &[Vec<i32>], coins_size: usize, coins_col_size: &[usize]) -> i32 {
    // The original code treats the bottom-right cell as (row, col),
    // so we define row and col as the last valid indices.
    let row = coins_size - 1;        // last row index
    let col = coins_col_size[0] - 1; // last column index (assuming rectangular grid)

    // Use a 3D DP array: dp[row+1][col+1][3]
    // where dp[j][i][0], dp[j][i][1], dp[j][i][2] store intermediate states.
    let mut dp = vec![vec![[0; 3]; col + 1]; row + 1];

    // Initialize the bottom-right corner equivalent to C: dp[row][col][:]
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = i32::max(coins[row][col], 0);
    dp[row][col][2] = i32::max(coins[row][col], 0);

    // Fill the last row from right to left
    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = i32::max(dp[row][i + 1][0], dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = i32::max(dp[row][i + 1][1], dp[row][i + 1][2] + coins[row][i]);
    }

    // Fill the last column from bottom to top
    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = i32::max(dp[i + 1][col][0], dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = i32::max(dp[i + 1][col][1], dp[i + 1][col][2] + coins[i][col]);
    }

    // Fill the rest of the DP table in reverse order
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = i32::max(dp[j + 1][i][0], dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = i32::max(
                i32::max(dp[j][i + 1][0], dp[j][i + 1][1] + coins[j][i]),
                i32::max(dp[j + 1][i][0], dp[j + 1][i][1] + coins[j][i]),
            );
            dp[j][i][2] = i32::max(
                i32::max(dp[j][i + 1][1], dp[j][i + 1][2] + coins[j][i]),
                i32::max(dp[j + 1][i][1], dp[j + 1][i][2] + coins[j][i]),
            );
        }
    }

    // The final answer is dp[0][0][2] in the original code
    dp[0][0][2]
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the first line containing n and m
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let mut parts = line.trim().split_whitespace();
    let n: usize = parts.next().ok_or("Missing 'n'")?.parse()?;
    let m: usize = parts.next().ok_or("Missing 'm'")?.parse()?;

    // Read the next n lines, each containing m integers
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        line.clear();
        stdin.lock().read_line(&mut line)?;
        let row_vals = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        if row_vals.len() != m {
            return Err("Incorrect number of columns in input row".into());
        }
        coins.push(row_vals);
    }

    // Prepare the coinsColSize array (in C it was an int array: all equal to m)
    let coins_col_size = vec![m; n];

    // Compute the result using our translated DP function
    let result = maximum_amount(&coins, n, &coins_col_size);

    // Output the result exactly as the C code does
    println!("{}", result);

    Ok(())
}