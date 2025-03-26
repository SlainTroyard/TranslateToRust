// Problem: Weekly Contest 435 Problem 3

use std::cmp::min;
use std::io::{self, BufRead};
use std::num::ParseIntError;

// Utility function to calculate GCD (Greatest Common Divisor) using Euclid's algorithm
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

struct Solution;

impl Solution {
    pub fn minimum_increments(nums: Vec<i64>, target: Vec<i64>) -> i64 {
        let n = nums.len();
        let m = target.len();
        let mut g = vec![1i64; 1 << m];

        // Precompute the lcm values for all subsets of the target array
        for i in 0..(1 << m) {
            for j in 0..m {
                if i >> j & 1 == 1 {
                    g[i] = g[i] / gcd(g[i], target[j]) * target[j];
                }
            }
        }

        const INF: i64 = 1_000_000_000_000_000_000;
        let mut f = vec![vec![INF; 1 << m]; 2];
        f[0][0] = 0;

        for i in 1..=n {
            let cur = i & 1;
            let prev = cur ^ 1;

            f[cur].clone_from_slice(&f[prev]); // Copy previous state into current state
            for j in 0..(1 << m) {
                for k in 0..=j {
                    if k & j != k {
                        continue;
                    }
                    let v = (nums[i - 1] + g[k] - 1) / g[k] * g[k] - nums[i - 1];
                    f[cur][j] = min(f[cur][j], f[prev][j ^ k] + v);
                }
            }
        }

        f[n & 1][(1 << m) - 1]
    }
}

fn main() -> Result<(), ParseIntError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the dimensions of the input arrays: n and m
    let first_line = lines.next().unwrap()?;
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse::<usize>())
        .collect::<Result<_, _>>()?;
    let n = dimensions[0];
    let m = dimensions[1];

    // Read the nums array
    let second_line = lines.next().unwrap()?;
    let nums: Vec<i64> = second_line
        .split_whitespace()
        .map(|x| x.parse::<i64>())
        .collect::<Result<_, _>>()?;

    // Read the target array
    let third_line = lines.next().unwrap()?;
    let target: Vec<i64> = third_line
        .split_whitespace()
        .map(|x| x.parse::<i64>())
        .collect::<Result<_, _>>()?;

    // Call the solution
    let result = Solution::minimum_increments(nums, target);

    // Output the result
    println!("{}", result);

    Ok(())
}