// Weekly Contest 426 Problem 2 in Rust
// -----------------------------------
// This program reads an integer N from stdin, then reads N integers
// into a vector, finds the "largest outlier" using the same logic
// as the C++ code, and prints the result to stdout.

use std::collections::HashMap;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn get_largest_outlier(&self, nums: &[i32]) -> i32 {
        // Count the occurrences of each number and compute the sum
        let mut counts = HashMap::new();
        let mut sum: i64 = 0;
        for &num in nums {
            *counts.entry(num).or_insert(0) += 1;
            sum += num as i64;
        }

        // Collect unique candidates
        let mut candidates: Vec<i32> = counts.keys().cloned().collect();
        // Sort in descending order
        candidates.sort_by(|a, b| b.cmp(a));

        // Check each candidate if it satisfies the (sum - candidate) / 2 condition
        for &candidate in &candidates {
            let diff = sum - candidate as i64;
            let d = diff / 2;
            let m = diff % 2;
            if m == 0 {
                if let Some(&count_d) = counts.get(&(d as i32)) {
                    // If d != candidate or candidate occurs more than once
                    if d != candidate as i64 || count_d > 1 {
                        return candidate;
                    }
                }
            }
        }

        // Return -1 if no candidate matches
        -1
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the number of elements N
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let n: usize = line.trim().parse()?;

    // Read the N integers from stdin
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        line.clear();
        let bytes_read = io::stdin().read_line(&mut line)?;
        // If there's no more input, break
        if bytes_read == 0 {
            break;
        }
        for token in line.split_whitespace() {
            let val: i32 = token.parse()?;
            nums.push(val);
            if nums.len() == n {
                break;
            }
        }
    }

    // Create a solution instance
    let solution = Solution;
    // Compute the result
    let result = solution.get_largest_outlier(&nums);

    // Print the result (no extra formatting)
    println!("{}", result);

    Ok(())
}