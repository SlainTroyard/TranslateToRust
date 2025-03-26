use std::io::{self, BufRead, Write};

fn main() {
    // Read the dimensions of the grid
    let (n, m) = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let nums: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.trim().parse().expect("Please type a number!"))
            .collect();
        (nums[0], nums[1])
    };

    // Read the grid of coins
    let mut coins = vec![vec![0; m]; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        coins[i] = input
            .split_whitespace()
            .map(|s| s.trim().parse().expect("Please type a number!"))
            .collect();
    }

    // Create an instance of the Solution and compute the result
    let solution = Solution;
    let result = solution.maximum_amount(&coins);

    // Print the result
    writeln!(io::stdout(), "{}", result).expect("Failed to write to stdout");
}

struct Solution;

impl Solution {
    fn maximum_amount(coins: &Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        const INF: i64 = 1_000_000_000_000_000_000; // 1e18
        let mut f = vec![vec![vec![-INF; 3]; m]; n];

        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        for i in 0..n {
            for j in 0..m {
                for k in 0..3 {
                    if i > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i - 1][j][k] + coins[i][j] as i64);
                    }
                    if j > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i][j - 1][k] + coins[i][j] as i64);
                    }
                }
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

        let mut ans = -INF;
        for k in 0..3 {
            ans = ans.max(f[n - 1][m - 1][k]);
        }
        ans
    }
}