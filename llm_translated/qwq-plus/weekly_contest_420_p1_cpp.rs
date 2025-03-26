use std::io;

/// Represents the solution for LeetCode Weekly Contest 420 Problem 1.
/// Translated from C++ to idiomatic Rust while preserving algorithm logic.
pub struct Solution;

impl Solution {
    /// Generates a sequence of strings based on the target string.
    ///
    /// # Arguments
    ///
    /// * `target` - The target string to generate the sequence from.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the sequence.
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();
        for c in target.chars() {
            // Add a placeholder 'a' to the current string
            s.push('a');
            // Iterate from 'a' to the current character (inclusive)
            for current_char in 'a'..=c {
                // Modify the last character of the string to current_char
                let last_byte = s.as_bytes_mut().last_mut().unwrap();
                *last_byte = current_char as u8;
                ans.push(s.clone());
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input from stdin");

    // Extract the first word from the input as the target string
    let target = input
        .trim()
        .split_whitespace()
        .next()
        .expect("No target string provided")
        .to_string();

    // Generate the sequence and print the results
    let ans = Solution::string_sequence(target);
    for s in &ans {
        print!("{} ", s);
    }
    println!(); // Ensure a newline at the end as per original C++ code
}