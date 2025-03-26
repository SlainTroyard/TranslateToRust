use std::io::{self, BufRead};

fn maximum_amount(coins: &[Vec<i32>]) -> i32 {
    let row = coins.len() - 1;
    let col = coins[0].len() - 1;
    let mut dp = vec![vec![vec![0; 3]; col + 1]; row + 1];

    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = coins[row][col].max(0);
    dp[row][col][2] = coins[row][col].max(0);

    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = dp[row][i + 1][0].max(dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = dp[row][i + 1][1].max(dp[row][i + 1][2] + coins[row][i]);
    }

    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = dp[i + 1][col][0].max(dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = dp[i + 1][col][1].max(dp[i + 1][col][2] + coins[i][col]);
    }

    for j in (0..row).rev() {
        for i in (0..col).rev() {
            dp[j][i][0] = dp[j + 1][i][0].max(dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = dp[j][i + 1][0].max(dp[j][i + 1][1] + coins[j][i])
                .max(dp[j + 1][i][0].max(dp[j + 1][i][1] + coins[j][i]));
            dp[j][i][2] = dp[j][i + 1][1].max(dp[j][i + 1][2] + coins[j][i])
                .max(dp[j + 1][i][1].max(dp[j + 1][i][2] + coins[j][i]));
        }
    }

    dp[0][0][2]
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap()?;
    let mut nm = first_line.split_whitespace();
    let n: usize = nm.next().unwrap().parse().unwrap();
    let m: usize = nm.next().unwrap().parse().unwrap();

    // Read coins
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        coins.push(row);
    }

    // Calculate and print the result
    let result = maximum_amount(&coins);
    println!("{}", result);

    Ok(())
}