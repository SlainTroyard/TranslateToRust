use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    fn maximum_amount(coins: &Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        
        // Initialize 3D DP array with -INF
        let mut f = vec![vec![vec![-INF; 3]; m]; n];
        
        // Initialize starting position
        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        for i in 0..n {
            for j in 0..m {
                // Case 1: Take the coin at current cell
                for k in 0..3 {
                    if i > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i - 1][j][k] + coins[i][j] as i64);
                    }
                    if j > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i][j - 1][k] + coins[i][j] as i64);
                    }
                }
                
                // Case 2: Skip the coin at current cell (can only do this if we've skipped < 2 times)
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

        // Find the maximum value among all possible skip counts at the destination
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
    
    // Parse n and m
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    // Parse the coins grid
    let mut coins = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        for j in 0..m {
            coins[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }
    
    // Solve and output
    let solution = Solution::maximum_amount(&coins);
    println!("{}", solution);
}