use std::io::{self, Read};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i64>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();
        const INF: i64 = 1e18 as i64;

        // Initialize a 3D dp array with -INF
        let mut dp = vec![vec![vec![-INF; 3]; m]; n];
        dp[0][0][0] = coins[0][0]; // Start with the first cell
        dp[0][0][1] = 0;          // One "skip" used

        for i in 0..n {
            for j in 0..m {
                for k in 0..3 {
                    // Transition without using any skip
                    if i > 0 {
                        dp[i][j][k] = max(dp[i][j][k], dp[i - 1][j][k] + coins[i][j]);
                    }
                    if j > 0 {
                        dp[i][j][k] = max(dp[i][j][k], dp[i][j - 1][k] + coins[i][j]);
                    }
                }
                for k in 1..3 {
                    // Transition using a skip
                    if i > 0 {
                        dp[i][j][k] = max(dp[i][j][k], dp[i - 1][j][k - 1]);
                    }
                    if j > 0 {
                        dp[i][j][k] = max(dp[i][j][k], dp[i][j - 1][k - 1]);
                    }
                }
            }
        }

        // Find the maximum value in the last cell with at most 2 skips
        let mut ans = -INF;
        for k in 0..3 {
            ans = max(ans, dp[n - 1][m - 1][k]);
        }
        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Parse the input
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let n: usize = dimensions.next().unwrap().parse().unwrap();
    let m: usize = dimensions.next().unwrap().parse().unwrap();

    let mut coins = vec![vec![0; m]; n];
    for (i, line) in lines.enumerate() {
        let values = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        for (j, value) in values.enumerate() {
            coins[i][j] = value;
        }
    }

    // Call the solution function
    let solution = Solution;
    let result = solution.maximum_amount(coins);

    // Print the result
    println!("{}", result);
}