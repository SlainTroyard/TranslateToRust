use std::cmp::max;
use std::io;

struct Solution {}

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = if n > 0 { coins[0].len() } else { 0 };

        if n == 0 || m == 0 {
            return 0;
        }

        const INF: i64 = i64::MIN / 2; // To avoid potential overflow during addition
        let mut f = vec![vec![vec![INF; 3]; m]; n];

        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;
        f[0][0][2] = 0; // Initialize f[0][0][k] for all k for consistency. Although f[0][0][1] = 0 is from the original code, and f[0][0][2] is not explicitly initialized, we should initialize it to 0 or -INF. Let's use 0 for consistency with f[0][0][1].

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

        let mut ans = INF;
        for k in 0..3 {
            ans = max(ans, f[n - 1][m - 1][k]);
        }
        if ans == INF { // Handle the case where no path is possible (all coins are negative and we are not allowed to take any). Not needed for this problem as coins are non-negative in constraints.
            0
        } else {
            ans
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut coins: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        coins[i] = row;
    }

    let solution = Solution {};
    println!("{}", solution.maximum_amount(coins));
}