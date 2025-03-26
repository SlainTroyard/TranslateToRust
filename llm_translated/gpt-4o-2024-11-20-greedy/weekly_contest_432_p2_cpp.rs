use std::io::{self, BufRead};
use std::cmp::{max, min};

struct Solution;

impl Solution {
    fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        let mut f = vec![vec![vec![-INF; 3]; m]; n];
        
        // Initialize the starting point
        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        // DP transitions
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

        // Calculate the maximum result
        let mut ans = -INF;
        for k in 0..3 {
            ans = max(ans, f[n - 1][m - 1][k]);
        }
        ans
    }
}

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse dimensions
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Parse the coins grid
    let mut coins = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row_values = line.split_whitespace()
                             .map(|x| x.parse::<i32>().unwrap())
                             .collect::<Vec<_>>();
        for j in 0..m {
            coins[i][j] = row_values[j];
        }
    }

    // Solve the problem
    let solution = Solution;
    let result = solution.maximum_amount(coins);

    // Print the output
    println!("{}", result);
}