// Equivalent Rust translation of the C++ code from LeetCode Weekly Contest 420 Problem 3
//
// Requirements met:
// 1. Translates the entire file (including main and I/O) as a complete program
// 2. Preserves the exact algorithm logic
// 3. Uses idiomatic Rust with proper error handling
// 4. Maintains the EXACT SAME stdin/stdout format
// 5. Includes helpful comments

use std::io::{self, BufRead};

const MX: usize = 1_000_001;

struct Solution {
    // LPF array, where LPF[x] = least prime factor of x
    lpf: [usize; MX],
}

impl Solution {
    // Constructor-like function to initialize the LPF array
    fn new() -> Self {
        let mut s = Self { lpf: [0; MX] };
        for i in 2..MX {
            if s.lpf[i] == 0 {
                // For each multiple of i, set the least prime factor if not already set
                let mut j = i;
                while j < MX {
                    if s.lpf[j] == 0 {
                        s.lpf[j] = i;
                    }
                    j += i;
                }
            }
        }
        s
    }

    // Translated minOperations method
    // This replicates the logic:
    //  - Traverse from end to start
    //  - If nums[i] > nums[i+1], replace nums[i] with LPF[nums[i]]
    //  - Check again; if still greater, return -1
    //  - Count each change as one operation
    fn min_operations(&mut self, nums: &mut [usize]) -> i32 {
        let mut ans = 0;
        // Iterate backwards from the second-to-last element down to 0
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                nums[i] = self.lpf[nums[i]];
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare to read from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n (number of elements)
    let n: usize = {
        let first_line = lines
            .next()
            .ok_or("Missing input for n")??;
        first_line.trim().parse()?
    };

    // Read n integers
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        let line = lines
            .next()
            .ok_or("Not enough input lines for n numbers")??;
        for token in line.split_whitespace() {
            nums.push(token.parse()?);
            if nums.len() == n {
                break;
            }
        }
    }

    // Initialize the solution (which creates the LPF array)
    let mut sol = Solution::new();

    // Compute the minimal operations
    let ans = sol.min_operations(&mut nums);

    // Print the result
    println!("{}", ans);

    Ok(())
}