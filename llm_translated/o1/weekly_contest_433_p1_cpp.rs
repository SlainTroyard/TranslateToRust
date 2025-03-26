// Problem: Weekly Contest 433 Problem 1
// Translated from C++ to Rust

use std::io::{self, BufRead};

/// A struct to encapsulate our solution methods.
struct Solution;

impl Solution {
    /// Computes the sum of certain subarray segments as defined by the problem.
    /// 
    /// - nums: A vector of integers.
    /// 
    /// Returns the accumulated sum of (s[i+1] - s[max(i - nums[i], 0)]) for i in [0..n).
    fn subarray_sum(&self, nums: &Vec<i32>) -> i32 {
        let n = nums.len();
        
        // s will store the partial sums of nums, with s[0] = 0
        let mut s = vec![0i32; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }

        let mut ans = 0i32;
        // For each i, compute the subarray sum difference:
        // s[i+1] - s[ max(i - nums[i], 0) ]
        for i in 0..n {
            let i_i32 = i as i32;
            // saturating_sub avoids negative values, matching max(i - nums[i], 0) in C++
            let j = i_i32.saturating_sub(nums[i]) as usize;
            ans += s[i + 1] - s[j];
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the integer n
    let n_line = lines.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "No input"))??;
    let n: usize = n_line.trim().parse().expect("Failed to parse n");

    // Read n integers (possibly split across lines)
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        if let Some(line) = lines.next() {
            let line = line?;
            for val in line.split_whitespace() {
                if nums.len() < n {
                    nums.push(val.parse().expect("Failed to parse integer"));
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }

    // Create a solution instance and compute the result
    let solution = Solution;
    let result = solution.subarray_sum(&nums);

    // Print the result
    println!("{}", result);

    Ok(())
}