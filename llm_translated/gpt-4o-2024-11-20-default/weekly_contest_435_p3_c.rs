use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::min;

/// Calculate the greatest common divisor of two numbers.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Main function implementing the algorithm.
fn minimum_increments(nums: &[i32], targets: &[i32]) -> i32 {
    let n = nums.len();
    let m = targets.len();
    
    // Precompute g array for subset divisibility.
    let subset_count = 1 << m;
    let mut g = vec![1i64; subset_count];
    for i in 0..subset_count {
        for j in 0..m {
            if (i >> j) & 1 == 1 {
                g[i] = g[i] / gcd(g[i], targets[j] as i64) * targets[j] as i64;
            }
        }
    }
    
    // Dynamic programming arrays for the algorithm.
    const INF: i64 = 1e18 as i64;
    let mut dp = vec![vec![INF; subset_count]; 2];
    dp[0][0] = 0;

    // DP process
    for i in 1..=n {
        let current = i & 1;
        let previous = current ^ 1;

        // Copy previous DP values to current.
        for j in 0..subset_count {
            dp[current][j] = dp[previous][j];
        }

        // Update DP values using nums and gcd subsets.
        for j in 0..subset_count {
            let mut k = j;
            while k > 0 {
                let v = ((((nums[i - 1] as i64 + g[k] - 1) / g[k]) * g[k]) - nums[i - 1] as i64);
                dp[current][j] = min(dp[current][j], dp[previous][j ^ k] + v);
                k = (k - 1) & j;
            }
        }
    }    

    // The result is the minimum cost to cover the full subset `(1 << m) - 1`.
    dp[n & 1][subset_count - 1] as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = parts[0];
    let m = parts[1];

    // Read nums array
    let second_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read target array
    let third_line = lines.next().unwrap().unwrap();
    let targets: Vec<i32> = third_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure sizes match
    assert_eq!(nums.len(), n);
    assert_eq!(targets.len(), m);

    // Compute and output the result
    let result = minimum_increments(&nums, &targets);
    println!("{}", result);
}