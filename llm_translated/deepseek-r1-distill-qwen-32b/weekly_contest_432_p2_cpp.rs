use std::io;

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
        let n = coins.len();
        if n == 0 {
            return 0;
        }
        let m = coins[0].len();
        if m == 0 {
            return 0;
        }

        let mut f = vec![vec![[i64::MIN; 3]; m]; n];
        f[0][0][0] = coins[0][0] as i64;
        f[0][0][1] = 0;

        for i in 0..n {
            for j in 0..m {
                // Update for k=0,1,2
                for k in 0..3 {
                    if i > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i-1][j][k] + coins[i][j] as i64);
                    }
                    if j > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i][j-1][k] + coins[i][j] as i64);
                    }
                }
                // Update for k=1,2
                for k in 1..3 {
                    if i > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i-1][j][k-1]);
                    }
                    if j > 0 {
                        f[i][j][k] = f[i][j][k].max(f[i][j-1][k-1]);
                    }
                }
            }
        }

        let mut ans = i64::MIN;
        for k in 0..3 {
            ans = ans.max(f[n-1][m-1][k]);
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let mut coins = vec![vec![0; m as usize]; n as usize];
    for i in 0..n as usize {
        for j in 0..m as usize {
            coins[i][j] = tokens.next().unwrap();
        }
    }
    let solution = Solution {};
    println!("{}", solution.maximum_amount(coins));
}