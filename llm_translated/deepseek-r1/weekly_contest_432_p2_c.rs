use std::io::{self, Read};

fn maximum_amount(coins: &[Vec<i32>], coins_size: usize, coins_col_size: &[usize]) -> i32 {
    let rows = coins_size;
    let cols = coins_col_size[0];
    let row = rows - 1;
    let col = cols - 1;

    // Initialize 3D DP array with dimensions (rows, cols, 3)
    let mut dp = vec![vec![vec![0; 3]; cols]; rows];

    // Base case: bottom-right cell
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = coins[row][col].max(0);
    dp[row][col][2] = coins[row][col].max(0);

    // Fill last row (moving left)
    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = dp[row][i + 1][0].max(dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = dp[row][i + 1][1].max(dp[row][i + 1][2] + coins[row][i]);
    }

    // Fill last column (moving up)
    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = dp[i + 1][col][0].max(dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = dp[i + 1][col][1].max(dp[i + 1][col][2] + coins[i][col]);
    }

    // Fill remaining grid (bottom-up, right-left)
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = dp[j + 1][i][0].max(dp[j][i + 1][0]) + coins[j][i];
            
            let left_choice1 = dp[j][i + 1][0].max(dp[j][i + 1][1] + coins[j][i]);
            let up_choice1 = dp[j + 1][i][0].max(dp[j + 1][i][1] + coins[j][i]);
            dp[j][i][1] = left_choice1.max(up_choice1);
            
            let left_choice2 = dp[j][i + 1][1].max(dp[j][i + 1][2] + coins[j][i]);
            let up_choice2 = dp[j + 1][i][1].max(dp[j + 1][i][2] + coins[j][i]);
            dp[j][i][2] = left_choice2.max(up_choice2);
        }
    }

    dp[0][0][2]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Read grid dimensions
    let n: usize = tokens.next().expect("Missing n").parse().expect("Invalid n");
    let m: usize = tokens.next().expect("Missing m").parse().expect("Invalid m");

    // Read coin grid
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = Vec::with_capacity(m);
        for _ in 0..m {
            let val: i32 = tokens.next().expect("Insufficient coin values").parse().expect("Invalid coin value");
            row.push(val);
        }
        coins.push(row);
    }

    // All rows have the same column size (m)
    let coins_col_size = vec![m; n];

    println!("{}", maximum_amount(&coins, n, &coins_col_size));
}