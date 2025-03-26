use std::cmp::max;
use std::io;

struct Solution {}

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        let m = coins[0].len();

        const INF: i64 = 1_000_000_000_000_000_000; // 1e18 as i64

        let mut f = vec![vec![vec![-INF; 3]; m]; n];

        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

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

        let mut ans = -INF;
        for k in 0..3 {
            ans = max(ans, f[n - 1][m - 1][k]);
        }
        ans
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut iter = n.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut coins: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let row: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        coins.push(row);
    }

    let solution = Solution {};
    println!("{}", solution.maximum_amount(coins));
}