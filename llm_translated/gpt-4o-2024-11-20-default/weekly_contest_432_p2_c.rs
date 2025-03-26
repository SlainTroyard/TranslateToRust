use std::cmp::max;
use std::io;

fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
    let rows = coins.len();
    let cols = coins[0].len();
    
    let mut dp = vec![vec![vec![0; 3]; cols]; rows];
    
    dp[rows - 1][cols - 1][0] = coins[rows - 1][cols - 1];
    dp[rows - 1][cols - 1][1] = max(coins[rows - 1][cols - 1], 0);
    dp[rows - 1][cols - 1][2] = max(coins[rows - 1][cols - 1], 0);
    
    for col in (0..cols - 1).rev() {
        dp[rows - 1][col][0] = dp[rows - 1][col + 1][0] + coins[rows - 1][col];
        dp[rows - 1][col][1] = max(dp[rows - 1][col + 1][0], dp[rows - 1][col + 1][1] + coins[rows - 1][col]);
        dp[rows - 1][col][2] = max(dp[rows - 1][col + 1][1], dp[rows - 1][col + 1][2] + coins[rows - 1][col]);
    }
    
    for row in (0..rows - 1).rev() {
        dp[row][cols - 1][0] = dp[row + 1][cols - 1][0] + coins[row][cols - 1];
        dp[row][cols - 1][1] = max(dp[row + 1][cols - 1][0], dp[row + 1][cols - 1][1] + coins[row][cols - 1]);
        dp[row][cols - 1][2] = max(dp[row + 1][cols - 1][1], dp[row + 1][cols - 1][2] + coins[row][cols - 1]);
    }
    
    for row in (0..rows - 1).rev() {
        for col in (0..cols - 1).rev() {
            dp[row][col][0] = max(dp[row + 1][col][0], dp[row][col + 1][0]) + coins[row][col];
            dp[row][col][1] = max(
                max(dp[row][col + 1][0], dp[row][col + 1][1] + coins[row][col]),
                max(dp[row + 1][col][0], dp[row + 1][col][1] + coins[row][col]),
            );
            dp[row][col][2] = max(
                max(dp[row][col + 1][1], dp[row][col + 1][2] + coins[row][col]),
                max(dp[row + 1][col][1], dp[row + 1][col][2] + coins[row][col]),
            );
        }
    }
    
    dp[0][0][2]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut dimensions = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = dimensions.next().unwrap();
    let m = dimensions.next().unwrap();
    
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        coins.push(
            input
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }
    
    println!("{}", maximum_amount(coins));
}