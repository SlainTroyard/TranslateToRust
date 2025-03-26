// Problem: Weekly Contest 423 Problem 3
// Translated from C to Rust, preserving the same logic and I/O format.

use std::io::{self, BufRead};

/// Computes the sum of "good subsequences" in the given array (modified in-place),
/// with all calculations done modulo 1_000_000_007.
fn sum_of_good_subsequences(nums: &mut [i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    // We use fixed-size vectors to match the C code's arrays of size 100003
    let mut cnt = vec![0_i64; 100_003];
    let mut sum = vec![0_i64; 100_003];
    let mut ans = 0_i64;

    for val in nums.iter_mut() {
        // In C: x = ++nums[i]
        *val += 1;
        let x = *val as usize;

        // c = cnt[x - 1] + 1 + cnt[x + 1]
        let c = (cnt[x - 1] + 1 + cnt[x + 1]) % MOD;
        // cnt[x] = (cnt[x] + c) % MOD
        cnt[x] = (cnt[x] + c) % MOD;

        // s = c * (x - 1) + sum[x - 1] + sum[x + 1]
        let s = (c * (x as i64 - 1) + sum[x - 1] + sum[x + 1]) % MOD;
        // sum[x] = (sum[x] + s) % MOD
        sum[x] = (sum[x] + s) % MOD;

        // ans = (ans + s) % MOD
        ans = (ans + s) % MOD;
    }
    ans as i32
}

fn main() {
    // Read input from stdin, token by token, to replicate the C code's scanf behavior
    let stdin = io::stdin();
    let tokens: Vec<i32> = stdin
        .lock()
        .lines()
        .flat_map(|line| {
            // Each line is split by whitespace into tokens
            line
                .unwrap_or_else(|_| String::new())
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
                .collect::<Vec<i32>>()
        })
        .collect();

    // The first token is the size of the array, n
    if tokens.is_empty() {
        // If no input for n, we mimic returning 1 like C code might on invalid input
        // (Though in practice, Rust would simply fail to parse.)
        eprintln!("Memory allocation failed or no valid input!");
        std::process::exit(1);
    }
    let n = tokens[0] as usize;

    // In C, "malloc" was used and then each element read with scanf.
    // Here, we rely on the tokens array. The next n tokens are the array elements.
    if tokens.len() < n + 1 {
        eprintln!("Not enough integers provided!");
        std::process::exit(1);
    }
    let mut nums = tokens[1..1 + n].to_vec();

    // Call the function and output the result
    let result = sum_of_good_subsequences(&mut nums);
    println!("{}", result);
}