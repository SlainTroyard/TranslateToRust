use std::io;

fn maximum_amount(coins: &Vec<Vec<i32>>) -> i32 {
    let rows = coins.len();
    let cols = coins[0].len();
    
    // Initialize DP table with dimensions [rows][cols][3]
    let mut dp = vec![vec![vec![0; 3]; cols]; rows];
    
    // Base case: bottom-right cell
    dp[rows-1][cols-1][0] = coins[rows-1][cols-1];
    dp[rows-1][cols-1][1] = coins[rows-1][cols-1].max(0);
    dp[rows-1][cols-1][2] = coins[rows-1][cols-1].max(0);
    
    // Fill last row
    for i in (0..cols-1).rev() {
        dp[rows-1][i][0] = dp[rows-1][i+1][0] + coins[rows-1][i];
        dp[rows-1][i][1] = dp[rows-1][i+1][0].max(dp[rows-1][i+1][1] + coins[rows-1][i]);
        dp[rows-1][i][2] = dp[rows-1][i+1][1].max(dp[rows-1][i+1][2] + coins[rows-1][i]);
    }
    
    // Fill last column
    for i in (0..rows-1).rev() {
        dp[i][cols-1][0] = dp[i+1][cols-1][0] + coins[i][cols-1];
        dp[i][cols-1][1] = dp[i+1][cols-1][0].max(dp[i+1][cols-1][1] + coins[i][cols-1]);
        dp[i][cols-1][2] = dp[i+1][cols-1][1].max(dp[i+1][cols-1][2] + coins[i][cols-1]);
    }
    
    // Fill remaining cells
    for j in (0..rows-1).rev() {
        for i in (0..cols-1).rev() {
            // State 0: can only come from below or right, take the max and add current coin
            dp[j][i][0] = dp[j+1][i][0].max(dp[j][i+1][0]) + coins[j][i];
            
            // State 1: can come from below or right, either take current coin from state 0 or add to state 1
            let option1 = dp[j][i+1][0].max(dp[j][i+1][1] + coins[j][i]);
            let option2 = dp[j+1][i][0].max(dp[j+1][i][1] + coins[j][i]);
            dp[j][i][1] = option1.max(option2);
            
            // State 2: can come from below or right, either take current coin from state 1 or add to state 2
            let option3 = dp[j][i+1][1].max(dp[j][i+1][2] + coins[j][i]);
            let option4 = dp[j+1][i][1].max(dp[j+1][i][2] + coins[j][i]);
            dp[j][i][2] = option3.max(option4);
        }
    }
    
    dp[0][0][2]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Invalid input");
    let m: usize = iter.next().unwrap().parse().expect("Invalid input");
    
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row = String::new();
        io::stdin().read_line(&mut row).expect("Failed to read input");
        let row: Vec<i32> = row.split_whitespace()
                               .map(|x| x.parse().expect("Invalid input"))
                               .collect();
        coins.push(row);
    }
    
    let result = maximum_amount(&coins);
    println!("{}", result);
}