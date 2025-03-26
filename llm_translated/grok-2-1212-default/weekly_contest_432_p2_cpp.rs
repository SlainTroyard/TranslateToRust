use std::io::{self, BufRead};

fn maximum_amount(coins: &Vec<Vec<i32>>) -> i64 {
    let n = coins.len();
    let m = coins[0].len();

    const INF: i64 = 1e18 as i64;
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let nm_line = lines.next().unwrap()?;
    let mut nm_iter = nm_line.split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse().unwrap();
    let m: usize = nm_iter.next().unwrap().parse().unwrap();

    // Read coins
    let mut coins = Vec::with_capacity(n);
    for _ in 0..n {
        let coin_line = lines.next().unwrap()?;
        let row: Vec<i32> = coin_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        coins.push(row);
    }

    // Calculate and print result
    let result = maximum_amount(&coins);
    println!("{}", result);

    Ok(())
}