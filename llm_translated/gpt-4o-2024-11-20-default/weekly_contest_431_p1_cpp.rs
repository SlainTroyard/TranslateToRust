// Problem: Weekly Contest 431 Problem 1 (translated from C++ to Rust)

use std::io::{self, BufRead};
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> usize {
        let n = nums.len();
        let mut m = 0;

        // Determine the maximum value in nums
        for &x in &nums {
            m = max(m, x);
        }

        // Compute the list of prime factors for each number from 0 to m
        let mut fac = vec![vec![]; (m + 1) as usize];
        for i in 2..=m {
            if fac[i as usize].is_empty() {
                let mut j = i;
                while j <= m {
                    fac[j as usize].push(i);
                    j += i;
                }
            }
        }

        let mut ans = 2;
        let mut vis = vec![false; (m + 1) as usize];

        let mut j = 0;
        for i in 0..n {
            while j < n {
                // Check if nums[j] can be included in the subsequence
                let check = || {
                    for &p in &fac[nums[j] as usize] {
                        if vis[p as usize] {
                            return false;
                        }
                    }
                    true
                };
                if check() {
                    for &p in &fac[nums[j] as usize] {
                        vis[p as usize] = true;
                    }
                    j += 1;
                } else {
                    break;
                }
            }

            ans = max(ans, j - i);

            // Remove nums[i] from the current subsequence
            for &p in &fac[nums[i] as usize] {
                vis[p as usize] = false;
            }
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read the size of the nums vector
    let num_size: usize = input
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .parse()
        .expect("Expected valid number");

    // Read the nums vector
    let nums: Vec<i32> = input
        .next()
        .expect("Expected input")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Expected valid number"))
        .collect();

    assert_eq!(nums.len(), num_size, "Input size does not match the number of elements.");

    // Solve and output the result
    let solution = Solution;
    let result = solution.max_length(nums);

    // Print output to stdout
    println!("{}", result);
}