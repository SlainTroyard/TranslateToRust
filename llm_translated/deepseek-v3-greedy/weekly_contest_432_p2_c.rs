use std::cmp::max;
use std::io;

fn maximum_amount(coins: &Vec<Vec<i32>>) -> i32 {
    let row = coins.len() - 1;
    let col = coins[0].len() - 1;
    let mut dp = vec![vec![vec![0; 3]; col + 1]; row + 1];

    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = max(coins[row][col], 0);
    dp[row][col][2] = max(coins[row][col], 0);

    for i in (0..col).rev() {
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = max(dp[row][i + 1][0], dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = max(dp[row][i + 1][1], dp[row][i + 1][2] + coins[row][i]);
    }

    for i in (0..row).rev() {
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = max(dp[i + 1][col][0], dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = max(dp[i + 1][col][1], dp[i + 1][col][2] + coins[i][col]);
    }

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

    dp[0][0][2]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut dimensions = input.split_whitespace();
    let n: usize = dimensions.next().unwrap().parse().unwrap();
    let m: usize = dimensions.next().unwrap().parse().unwrap();

    let mut coins = vec![vec![0; m]; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut row = input.split_whitespace();
        for j in 0..m {
            coins[i][j] = row.next().unwrap().parse().unwrap();
        }
    }

    println!("{}", maximum_amount(&coins));
}