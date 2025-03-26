use std::io::{self, Read};

const INF: i64 = 1_000_000_000_000_000_000;

fn maximum_amount(coins: Vec<Vec<i32>>) -> i64 {
    let n = coins.len();
    let m = coins[0].len();

    // Initialize 3D DP array with -INF to represent unreachable states
    let mut f = vec![vec![vec![-INF; 3]; m]; n];
    // Starting state: collect first cell (k=0) or skip it (k=1)
    f[0][0][0] = coins[0][0] as i64;
    f[0][0][1] = 0;

    for i in 0..n {
        for j in 0..m {
            let current_coin = coins[i][j] as i64;

            // Update DP by moving from top or left while collecting current coin
            for k in 0..3 {
                if i > 0 {
                    f[i][j][k] = f[i][j][k].max(f[i-1][j][k] + current_coin);
                }
                if j > 0 {
                    f[i][j][k] = f[i][j][k].max(f[i][j-1][k] + current_coin);
                }
            }

            // Update DP by moving from top or left while skipping current coin
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

    // Best result is the maximum of all possible skip counts at destination
    (0..3).map(|k| f[n-1][m-1][k]).max().unwrap_or(-INF)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    // Read grid dimensions
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();

    // Read grid data in row-major order
    let coins_data: Vec<i32> = tokens
        .take(n * m)
        .map(|s| s.parse().unwrap())
        .collect();

    // Convert flat data to 2D vector
    let coins: Vec<Vec<i32>> = coins_data
        .chunks_exact(m)
        .map(|chunk| chunk.to_vec())
        .collect();

    // Compute and print result
    println!("{}", maximum_amount(coins));
}