use std::io;
use std::cmp::min;

// Function to calculate the greatest common divisor (GCD)
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let mut iter = n.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).expect("Failed to read line");
    let nums: Vec<i64> = nums_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut target_input = String::new();
    io::stdin().read_line(&mut target_input).expect("Failed to read line");
    let target: Vec<i64> = target_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = minimum_increments(nums, target);
    println!("{}", result);
}

fn minimum_increments(nums: Vec<i64>, target: Vec<i64>) -> i64 {
    let n = nums.len();
    let m = target.len();

    let mut g = vec![1; 1 << m];
    for i in 0..(1 << m) {
        g[i] = 1;
        for j in 0..m {
            if (i >> j) & 1 != 0 {
                g[i] = g[i] / gcd(g[i], target[j]) * target[j];
            }
        }
    }

    const INF: i64 = 1_000_000_000_000_000_000; // 1e18;
    let mut f = vec![vec![INF; 1 << m]; 2];
    f[0][0] = 0;

    for i in 1..=n {
        for j in 0..(1 << m) {
            f[i & 1][j] = f[(i & 1) ^ 1][j];
        }
        for j in 0..(1 << m) {
            let mut k = j;
            while k > 0 {
                let v = ((nums[i - 1] + g[k] - 1) / g[k]) * g[k] - nums[i - 1];
                f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
                k = (k - 1) & j;
            }
        }
    }

    f[n & 1][(1 << m) - 1]
}