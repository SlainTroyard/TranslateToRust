use std::io::{self, BufRead};

// A struct to encapsulate our solution methods.
struct Solution;

impl Solution {
    // Calculates the maximum amount according to the provided dynamic programming logic.
    fn maximum_amount(&self, coins: &[Vec<i32>]) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        // Use a large negative value (in place of -INF).
        const INF: i64 = 1_000_000_000_000_000_000;
        
        // 3D DP array: f[i][j][k]
        //   i in [0..n), j in [0..m), k in [0..3)
        // f[i][j][k] can hold the maximum sum obtainable at cell (i,j) with k "skips" used
        // (an interpretation of how the original transitions work).
        let mut f = vec![vec![vec![-INF; 3]; m]; n];

        // Initialize the starting cell:
        // f[0][0][0] = coins[0][0], f[0][0][1] = 0
        // Means we either pick up the coin at (0,0) or skip it.
        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        // Populate the DP table
        for i in 0..n {
            for j in 0..m {
                for k in 0..3 {
                    // If we come from above (i-1, j) or left (i, j-1),
                    // we can add coins[i][j] to what's already collected.
                    if i > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i - 1][j][k] + coins[i][j] as i64);
                    }
                    if j > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i][j - 1][k] + coins[i][j] as i64);
                    }
                }
                // If we decide to skip collecting coins[i][j] but have "skip capacity" left,
                // we can carry forward previous value without adding the new coin.
                for k in 1..3 {
                    if i > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i - 1][j][k - 1]);
                    }
                    if j > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i][j - 1][k - 1]);
                    }
                }
            }
        }

        // The result is the maximum among all "skip states" at the bottom-right cell.
        let mut ans = -INF;
        for k in 0..3 {
            ans = ans.max(f[n - 1][m - 1][k]);
        }
        ans
    }
}

// The main function handles I/O exactly as in the C++ sample.
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut tokens = Vec::new();

    // Read all lines from stdin, split by whitespace, and collect the tokens.
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        for token in line.split_whitespace() {
            tokens.push(token.to_string());
        }
    }

    // Parse n and m from the token list.
    let mut index = 0;
    let n: usize = tokens[index].parse().unwrap();
    index += 1;
    let m: usize = tokens[index].parse().unwrap();
    index += 1;

    // Read the coin values.
    let mut coins = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            coins[i][j] = tokens[index].parse().unwrap();
            index += 1;
        }
    }

    // Solve and print the result.
    let solution = Solution;
    let answer = solution.maximum_amount(&coins);
    println!("{}", answer);

    Ok(())
}