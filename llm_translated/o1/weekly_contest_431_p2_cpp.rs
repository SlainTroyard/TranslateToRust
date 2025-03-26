// Problem: Weekly Contest 431 Problem 2
// Translated from C++ to Rust

use std::io::{self, BufRead};
use std::error::Error;

/// A solution struct to encapsulate our logic.
struct Solution;

impl Solution {
    /// This function replicates the logic of the C++ calculateScore method.
    /// It uses 26 stacks (emulated by 26 Vecs) to track the positions of characters
    /// in the string. Whenever we find a matching "opposite" character (25 - c),
    /// we pop from that stack and add the distance to our answer.
    fn calculate_score(s: &str) -> i64 {
        // Each element of stacks is a Vec that will store indices where characters occur.
        let mut stacks: [Vec<usize>; 26] = Default::default();
        let mut ans = 0i64;

        for (i, ch) in s.chars().enumerate() {
            // Convert character 'a'..'z' to 0..25
            let c = (ch as u8 - b'a') as usize;
            // Check if the corresponding stack for "25 - c" is non-empty
            if !stacks[25 - c].is_empty() {
                // Pop the last index from the "opposite" stack
                let top_index = stacks[25 - c].pop().unwrap();
                // Add the distance between current index i and that popped index
                ans += i as i64 - top_index as i64;
            } else {
                // If opposite stack is empty, push current index onto stack[c]
                stacks[c].push(i);
            }
        }

        ans
    }
}

/// Reads a single token from stdin, ignoring any leading empty lines or extra spaces.
/// In C++, "cin >> s" reads exactly one whitespace-delimited token. This function
/// mimics that behavior by searching line by line for the first token.
fn read_token() -> Result<String, Box<dyn Error>> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if let Some(token) = line.split_whitespace().next() {
            return Ok(token.to_string());
        }
    }
    // If no token is found, return an empty string (mimicking a default construct).
    Ok(String::new())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read a single token from stdin
    let s = read_token()?;
    // Calculate score using the replicated logic
    let ans = Solution::calculate_score(&s);
    // Print the result to stdout, exactly as in the original code
    println!("{}", ans);
    Ok(())
}