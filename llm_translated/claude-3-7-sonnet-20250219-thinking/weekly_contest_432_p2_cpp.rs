use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        
        // 3D dp array initialized with -INF
        let mut f = vec![vec![vec![-INF; 3]; m]; n];
        
        // Base case initialization
        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        for i in 0..n {
            for j in 0..m {
                // Update states by taking the coin
                for k in 0..3 {
                    if i > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i-1][j][k] + coins[i][j] as i64);
                    }
                    if j > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i][j-1][k] + coins[i][j] as i64);
                    }
                }
                
                // Update states by skipping the coin
                for k in 1..3 {
                    if i > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i-1][j][k-1]);
                    }
                    if j > 0 {
                        f[i][j][k] = max(f[i][j][k], f[i][j-1][k-1]);
                    }
                }
            }
        }

        // Find maximum among all states at bottom-right
        let mut ans = -INF;
        for k in 0..3 {
            ans = max(ans, f[n-1][m-1][k]);
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read dimensions n and m
    let dimensions: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = dimensions[0];
    let m = dimensions[1];
    
    // Read coins grid
    let mut coins = vec![vec![0; m]; n];
    let mut i = 0;
    let mut j = 0;
    
    // Read all input lines
    for line in lines {
        let line = line.unwrap();
        // Parse each number in the line
        for num in line.split_whitespace() {
            coins[i][j] = num.parse().unwrap();
            j += 1;
            if j == m {
                j = 0;
                i += 1;
                if i == n {
                    break;
                }
            }
        }
        if i == n {
            break;
        }
    }
    
    // Get and print the result
    let result = Solution::maximum_amount(coins);
    println!("{}", result);
}