use std::io::{self, BufRead, Write};

fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
    let n = coins.len();
    let m = coins[0].len();
    const INF: i64 = 1_000_000_000_000_000_000 / 1000; // Originally 1e18, here using 1e18 as integer.
    // We declare dp as a 3D vector: dp[i][j][k] which stores maximum amount at cell (i,j) with exactly k moves skipping coin addition.
    let mut dp = vec![vec![vec![-INF; 3]; m]; n];
    
    // Initialization at cell (0,0)
    dp[0][0][0] = coins[0][0] as i64;  // take coin value
    dp[0][0][1] = 0;                   // not taking coin value

    // Iterate over the grid cells row by row, column by column.
    for i in 0..n {
        for j in 0..m {
            // First update for transitions where coins are added.
            for k in 0..3 {
                if i > 0 {
                    dp[i][j][k] = dp[i][j][k].max(dp[i - 1][j][k] + coins[i][j] as i64);
                }
                if j > 0 {
                    dp[i][j][k] = dp[i][j][k].max(dp[i][j - 1][k] + coins[i][j] as i64);
                }
            }
            // Then update for transitions where coin addition is not taken (k indicates not adding coin in this cell).
            for k in 1..3 {
                if i > 0 {
                    dp[i][j][k] = dp[i][j][k].max(dp[i - 1][j][k - 1]);
                }
                if j > 0 {
                    dp[i][j][k] = dp[i][j][k].max(dp[i][j - 1][k - 1]);
                }
            }
        }
    }
    
    // Get the best answer from cell (n-1, m-1) over all choices of k.
    let mut ans = -INF;
    for k in 0..3 {
        ans = ans.max(dp[n - 1][m - 1][k]);
    }
    ans
}

fn main() -> io::Result<()> {
    // Set up buffered input and output.
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut input_line = String::new();
    
    // Read first line for n and m.
    input_line.clear();
    reader.read_line(&mut input_line)?;
    // Parse n and m from the first line.
    let mut parts = input_line.split_whitespace();
    let n: usize = parts
        .next()
        .expect("Expected n")
        .parse()
        .expect("n should be an integer");
    let m: usize = parts
        .next()
        .expect("Expected m")
        .parse()
        .expect("m should be an integer");
    
    // Read coin grid: n rows with m integers per row.
    let mut coins: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        // It might be that each row is provided on separate line,
        // but if not, we can keep reading tokens.
        input_line.clear();
        // Read a line and keep reading if empty tokens
        while input_line.trim().is_empty() {
            if reader.read_line(&mut input_line)? == 0 {
                break;
            }
        }
        let tokens: Vec<&str> = input_line.split_whitespace().collect();
        // If tokens are less than m, we may need to read further lines.
        let mut idx = 0;
        while idx < tokens.len() && idx < m {
            coins[i][idx] = tokens[idx]
                .parse()
                .expect("Each coin value should be an integer");
            idx += 1;
        }
        // In case the row was split across multiple lines, keep reading until m numbers are collected.
        while idx < m {
            input_line.clear();
            if reader.read_line(&mut input_line)? == 0 {
                break;
            }
            let extra_tokens: Vec<&str> = input_line.split_whitespace().collect();
            for token in extra_tokens {
                if idx >= m {
                    break;
                }
                coins[i][idx] = token
                    .parse()
                    .expect("Each coin value should be an integer");
                idx += 1;
            }
        }
    }
    
    // Get the maximum amount using our algorithm, then write output.
    let result = maximum_amount(coins);
    writeln!(stdout, "{}", result)?;
    
    Ok(())
}