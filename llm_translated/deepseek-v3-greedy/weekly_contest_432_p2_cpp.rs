use std::cmp::max;
use std::io;

fn main() {
    // Read the dimensions of the grid
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let dimensions: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    let n = dimensions[0];
    let m = dimensions[1];

    // Read the grid of coins
    let mut coins = vec![vec![0; m]; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        coins[i] = row;
    }

    // Calculate the maximum amount of coins
    let result = maximum_amount(&coins);
    println!("{}", result);
}

fn maximum_amount(coins: &Vec<Vec<i32>>) -> i64 {
    let n = coins.len();
    let m = coins[0].len();

    const INF: i64 = 1e18 as i64;
    let mut f = vec![vec![vec![-INF; 3]; m]; n];

    // Initialize the starting position
    f[0][0][0] = coins[0][0] as i64;
    f[0][0][1] = 0;

    // Dynamic programming to fill the f array
    for i in 0..n {
        for j in 0..m {
            for k in 0..3 {
                if i > 0 {
                    f[i][j][k] = max(f[i][j][k], f[i - 1][j][k] + coins[i][j] as i64);
                }
                if j > 0 {
                    f[i][j][k] = max(f[i][j][k], f[i][j - 1][k] + coins[i][j] as i64);
                }
            }
            for k in 1..3 {
                if i > 0 {
                    f[i][j][k] = max(f[i][j][k], f[i - 1][j][k - 1]);
                }
                if j > 0 {
                    f[i][j][k] = max(f[i][j][k], f[i][j - 1][k - 1]);
                }
            }
        }
    }

    // Find the maximum value in the last cell
    let mut ans = -INF;
    for k in 0..3 {
        ans = max(ans, f[n - 1][m - 1][k]);
    }

    ans
}