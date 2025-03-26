// Translated from the C++ code for LeetCode Weekly Contest 425 Problem 2

use std::collections::HashMap;
use std::io::{self, BufRead};

/// A struct to encapsulate the solution logic
struct Solution;

impl Solution {
    /// Checks if it is possible to rearrange substrings of length `n/k`
    /// in string `s` to match string `t` when both are split into `k` equal parts.
    fn is_possible_to_rearrange(&self, s: &str, t: &str, k: usize) -> bool {
        let n = s.len();
        let size = n / k;

        // HashMap to count occurrences of each substring
        let mut mp: HashMap<String, i32> = HashMap::new();

        // Increase counts for each substring of `s`
        for i in (0..n).step_by(size) {
            let sub = &s[i..i + size];
            *mp.entry(sub.to_string()).or_insert(0) += 1;
        }

        // Decrease counts for each substring of `t`
        for i in (0..n).step_by(size) {
            let sub = &t[i..i + size];
            *mp.entry(sub.to_string()).or_insert(0) -= 1;
        }

        // If any substring count is not zero, then rearrangement isn't possible
        for value in mp.values() {
            if *value != 0 {
                return false;
            }
        }

        true
    }
}

fn main() -> io::Result<()> {
    // Read input lines (s, t, k) from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read string s
    let s = lines.next().unwrap_or(Ok(String::new()))?.trim().to_string();
    // Read string t
    let t = lines.next().unwrap_or(Ok(String::new()))?.trim().to_string();
    // Read integer k
    let k_str = lines.next().unwrap_or(Ok(String::new()))?.trim().to_string();
    let k: usize = k_str.parse().unwrap_or(0);

    // Create a Solution instance and check the condition
    let solution = Solution;
    let result = solution.is_possible_to_rearrange(&s, &t, k);

    // Print output exactly matching the original code's format
    if result {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}