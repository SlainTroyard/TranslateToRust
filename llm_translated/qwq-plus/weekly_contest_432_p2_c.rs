use std::io;

fn maximum_amount(coins: &Vec<Vec<i32>>) -> i32 {
    let n = coins.len();
    let m = coins[0].len();
    if n == 0 || m == 0 {
        return 0;
    }
    let row = n - 1;
    let col = m - 1;

    let mut dp = vec![vec![vec![0; 3]; m]; n];

    // Initialize bottom-right cell
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = coins[row][col].max(0);
    dp[row][col][2] = coins[row][col].max(0);

    // Process last row (row is fixed)
    for i in (0..col).rev() {
        let current_coin = coins[row][i];
        dp[row][i][0] = dp[row][i + 1][0] + current_coin;
        dp[row][i][1] = dp[row][i + 1][0].max(dp[row][i + 1][1] + current_coin);
        dp[row][i][2] = dp[row][i + 1][1].max(dp[row][i + 1][2] + current_coin);
    }

    // Process last column (column is fixed)
    for j in (0..row).rev() {
        let current_coin = coins[j][col];
        dp[j][col][0] = dp[j + 1][col][0] + current_coin;
        dp[j][col][1] = dp[j + 1][col][0].max(dp[j + 1][col][1] + current_coin);
        dp[j][col][2] = dp[j + 1][col][1].max(dp[j + 1][col][2] + current_coin);
    }

    // Process the rest of the cells
    for j in (0..row).rev() {
        for i in (0..col).rev() {
            let current_coin = coins[j][i];
            // Compute dp[j][i][0]
            let down = dp[j + 1][i][0];
            let right = dp[j][i + 1][0];
            dp[j][i][0] = down.max(right) + current_coin;

            // Compute dp[j][i][1]
            let right0 = dp[j][i + 1][0];
            let right1_plus_coin = dp[j][i + 1][1] + current_coin;
            let down0 = dp[j + 1][i][0];
            let down1_plus_coin = dp[j + 1][i][1] + current_coin;
            dp[j][i][1] = right0.max(right1_plus_coin).max(down0.max(down1_plus_coin));

            // Compute dp[j][i][2]
            let right1 = dp[j][i + 1][1];
            let right2_plus_coin = dp[j][i + 1][2] + current_coin;
            let down1 = dp[j + 1][i][1];
            let down2_plus_coin = dp[j + 1][i][2] + current_coin;
            dp[j][i][2] = right1.max(right2_plus_coin).max(down1.max(down2_plus_coin));
        }
    }

    dp[0][0][2]
}

fn main() {
    let (n, m) = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid integer"))
            .collect();
        (nums[0], nums[1])
    };

    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid integer"))
            .collect();
        coins.push(row);
    }

    // Ensure each row has exactly m elements
    for row in &coins {
        assert_eq!(row.len(), m);
    }

    let result = maximum_amount(&coins);
    println!("{}", result);
}