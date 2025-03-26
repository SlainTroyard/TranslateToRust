use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        const INF: i64 = 1_000_000_000_000_000_000;
        // Create a 3D array for dynamic programming
        let mut f = vec![vec![vec![-INF; 3]; m]; n];
        
        // Initialize the starting position
        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        for i in 0..n {
            for j in 0..m {
                // Update states by taking coins
                for k in 0..3 {
                    if i > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i - 1][j][k] + coins[i][j] as i64);
                    }
                    if j > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i][j - 1][k] + coins[i][j] as i64);
                    }
                }
                
                // Update states by skipping coins
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

        // Find the maximum value at the destination
        let mut ans = -INF;
        for k in 0..3 {
            ans = max(ans, f[n - 1][m - 1][k]);
        }
        
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Parse dimensions
    let dimensions = lines.next().unwrap().unwrap();
    let mut iter = dimensions.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Parse the coins grid
    let mut coins = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        for j in 0..m {
            coins[i][j] = values[j];
        }
    }
    
    // Solve and output the result
    let solution = Solution::maximum_amount(coins);
    println!("{}", solution);
}