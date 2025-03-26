// Problem: Weekly Contest 425 Problem 1
// Translated to Rust from the original C++ code, preserving logic and I/O format
use std::io::{self, BufRead};

// We create a struct to match the class structure in C++
struct Solution;

impl Solution {
    // Translated function from C++ to Rust
    fn minimum_sum_subarray(&self, nums: &[i32], l: i32, r: i32) -> i32 {
        let n = nums.len();
        let mut mini = i32::MAX;
        
        // Nested loops to check all subarrays
        for i in 0..n {
            let mut currsum = 0;
            for j in i..n {
                currsum += nums[j];
                let length = (j - i + 1) as i32;
                
                // Check if subarray length is in [l, r] and sum is positive
                if length >= l && length <= r && currsum > 0 {
                    if currsum < mini {
                        mini = currsum;
                    }
                }
            }
        }

        // If no valid subarray found, return -1
        if mini == i32::MAX {
            -1
        } else {
            mini
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    // Read the size of the array (n)
    stdin.read_line(&mut line)?;
    let n: usize = line.trim().parse().unwrap();
    line.clear();

    // Read n integers (may come from one or multiple lines)
    let mut nums = Vec::with_capacity(n);
    let mut count = 0;
    while count < n {
        line.clear();
        stdin.read_line(&mut line)?;
        for part in line.split_whitespace() {
            if count < n {
                nums.push(part.parse().unwrap());
                count += 1;
            } else {
                break;
            }
        }
    }

    // Read the range [l, r]
    line.clear();
    stdin.read_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let l: i32 = parts.next().unwrap().parse().unwrap();
    let r: i32 = parts.next().unwrap().parse().unwrap();

    // Create a solution instance and compute the minimum sum subarray
    let solution = Solution;
    let result = solution.minimum_sum_subarray(&nums, l, r);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}