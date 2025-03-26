use std::io;

const INF: i64 = 1_000_000_000_000_000_000; // 1e18

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        // Initialize DP table with -INF
        let mut f = vec![vec![vec![-INF; 3]; m]; n];
        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        for i in 0..n {
            for j in 0..m {
                // Update for all k (0, 1, 2)
                for k in 0..3 {
                    if i > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i - 1][j][k] + coins[i][j] as i64);
                    }
                    if j > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i][j - 1][k] + coins[i][j] as i64);
                    }
                }

                // Update for k >= 1 (1 and 2)
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

        // Find the maximum value at the bottom-right corner
        let mut ans = -INF;
        for k in 0..3 {
            ans = ans.max(f[n - 1][m - 1][k]);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut coins = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            coins[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }

    let result = Solution::maximum_amount(coins);
    println!("{}", result);
}