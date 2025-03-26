// Translation of C++ code from LeetCode Weekly Contest 434 Problem 3
// to idiomatic Rust with the exact same logic and I/O format.
//
// The original C++ code reads two integers n and k, then reads n integers
// into a vector, and finally computes and prints the result of solution.maxFrequency(nums, k).
//
// This Rust code preserves the same behavior and input/output format.
// It expects at least two integers (n, k) followed by n integers (nums).

use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // Translated logic of maxFrequency from C++ to Rust
    pub fn max_frequency(&self, nums: &[i32], k: i32) -> i32 {
        let mut f0 = 0;              // Counts how many times x == k
        let mut f1 = [0; 51];        // Array used to track frequency logic
        let mut max_f1 = 0;          // Stores maximum value in f1 array
        let mut f2 = 0;              // Tracks combined logic for x == k

        for &x in nums {
            let eq_k = if x == k { 1 } else { 0 };
            // Update f2 based on the maximum of current f2 or max_f1, then add 1 if x == k
            f2 = max(f2, max_f1) + eq_k;

            // f1[x] becomes the max of its current value or f0, then add 1
            f1[x as usize] = max(f1[x as usize], f0) + 1;

            // f0 increments by 1 if x == k
            f0 += eq_k;

            // Update max_f1 if needed
            max_f1 = max(max_f1, f1[x as usize]);
        }

        max(max_f1, f2)
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut tokens: Vec<i32> = Vec::new();

    // Read until we have at least 2 integers for n and k
    while tokens.len() < 2 {
        if let Some(line) = lines.next().transpose()? {
            tokens.extend(
                line.split_whitespace().filter_map(|s| s.parse::<i32>().ok())
            );
        } else {
            break;
        }
    }

    // If we still don't have enough input, exit
    if tokens.len() < 2 {
        eprintln!("Not enough input for n and k");
        return Ok(());
    }

    // n and k are the first two integers
    let n = tokens[0];
    let k = tokens[1];

    // Read until we have n more integers for nums
    while tokens.len() < (2 + n as usize) {
        if let Some(line) = lines.next().transpose()? {
            tokens.extend(
                line.split_whitespace().filter_map(|s| s.parse::<i32>().ok())
            );
        } else {
            break;
        }
    }

    // If we don't have enough nums, exit
    if tokens.len() < 2 + n as usize {
        eprintln!("Not enough input for nums");
        return Ok(());
    }

    // The next n integers are the nums array
    let nums = &tokens[2..2 + n as usize];
    let solution = Solution;
    let answer = solution.max_frequency(nums, k);

    // Print the result (same format as original C++ code)
    println!("{}", answer);

    Ok(())
}