use std::io;

fn main() {
    // Read the dimensions of the matrix
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let dims: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();
    let (n, m) = (dims[0], dims[1]);

    // Initialize the matrix
    let mut coins: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        let mut row_input = String::new();
        io::stdin().read_line(&mut row_input).expect("Failed to read line");
        for (j, val) in row_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Please type a number!"))
            .enumerate()
        {
            coins[i][j] = val;
        }
    }

    // Call the function to compute the maximum amount
    let result = maximum_amount(&coins);
    println!("{}", result);
}

/// Computes the maximum amount of coins that can be collected.
///
/// # Arguments
///
/// * `coins` - A 2D vector representing the grid of coins.
///
/// # Returns
///
/// * The maximum amount of coins that can be collected.
fn maximum_amount(coins: &Vec<Vec<i32>>) -> i32 {
    let row = coins.len() - 1;
    let col = coins[0].len() - 1;
    let mut dp = vec![vec![vec![0; 3]; col + 1]; row + 1];

    // Base case initialization
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = coins[row][col].max(0);
    dp[row][col][2] = coins[row][col].max(0);

    // Fill the last column
    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = dp[row][i + 1][0].max(dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = dp[row][i + 1][1].max(dp[row][i + 1][2] + coins[row][i]);
    }

    // Fill the last row
    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = dp[i + 1][col][0].max(dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = dp[i + 1][col][1].max(dp[i + 1][col][2] + coins[i][col]);
    }

    // Fill the rest of the dp table
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = dp[j + 1][i][0].max(dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = dp[j][i + 1][0]
                .max(dp[j][i + 1][1] + coins[j][i])
                .max(dp[j + 1][i][0])
                .max(dp[j + 1][i][1] + coins[j][i]);
            dp[j][i][2] = dp[j][i + 1][1]
                .max(dp[j][i + 1][2] + coins[j][i])
                .max(dp[j + 1][i][1])
                .max(dp[j + 1][i][2] + coins[j][i]);
        }
    }

    dp[0][0][2]
}