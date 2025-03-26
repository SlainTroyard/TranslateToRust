use std::io::{self, Read, Write};
use std::str::FromStr;

fn maximum_amount(coins: &[Vec<i32>]) -> i32 {
    let row = coins.len() - 1;
    let col = coins[0].len() - 1;
    let mut dp = vec![vec![vec![0; 3]; col + 1]; row + 1];

    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = i32::max(coins[row][col], 0);
    dp[row][col][2] = i32::max(coins[row][col], 0);

    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = i32::max(dp[row][i + 1][0], dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = i32::max(dp[row][i + 1][1], dp[row][i + 1][2] + coins[row][i]);
    }

    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = i32::max(dp[i + 1][col][0], dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = i32::max(dp[i + 1][col][1], dp[i + 1][col][2] + coins[i][col]);
    }

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

    dp[0][0][2]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut numbers = first_line.split_whitespace();

    let n = i32::from_str(numbers.next().unwrap()).unwrap();
    let m = i32::from_str(numbers.next().unwrap()).unwrap();

    let mut coins = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| i32::from_str(s).unwrap())
            .collect();
        coins.push(row);
    }

    let result = maximum_amount(&coins);
    writeln!(io::stdout(), "{}", result)?;

    Ok(())
}