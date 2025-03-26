// Problem: Weekly Contest 435 Problem 1
// Translated from C++ to Rust

use std::io::{self, BufRead};

// A struct to encapsulate our solution logic.
// This mimics the "Solution" class in C++.
struct Solution;

impl Solution {
    // This function calculates the difference between:
    //  - the maximum odd count of a character in s
    //  - the minimum even count of a character in s
    //  exactly like the original C++ code.
    fn max_difference(&self, s: &str) -> i32 {
        // Count array for characters 'a' to 'z'
        let mut cnt = [0i32; 26];

        // Populate the counts
        for b in s.bytes() {
            // Convert character to index (0..25)
            cnt[(b - b'a') as usize] += 1;
        }

        // Initialize max1 (maximum odd count) and
        // min0 (minimum even count) with i32::MAX for min0
        let mut max1 = 0;
        let mut min0 = i32::MAX;

        // Determine the maximum odd count and minimum even count
        for &c in &cnt {
            if c % 2 != 0 {
                // Update max1 if this count is the largest odd
                if c > max1 {
                    max1 = c;
                }
            } else if c != 0 {
                // Update min0 if this count is the smallest even (excluding zero)
                if c < min0 {
                    min0 = c;
                }
            }
        }

        // Return the difference as in the original C++ code
        max1 - min0
    }
}

fn main() -> io::Result<()> {
    // Read exactly one token from stdin, mirroring "cin >> s" in C++
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // We split on whitespace and take the first token
    let s = input.split_whitespace().next().unwrap_or("").to_string();

    // Create Solution instance and compute the result
    let sol = Solution;
    let answer = sol.max_difference(&s);

    // Print the result to stdout, matching the original output format
    println!("{}", answer);

    Ok(())
}