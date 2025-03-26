// Problem: Weekly Contest 437 Problem 1
use std::io::{self, Write};

struct Solution;

impl Solution {
    // Function to check if the string `s` contains a special substring of length `k`
    fn has_special_substring(s: &str, k: usize) -> bool {
        let n = s.len();
        let mut cnt = 0;

        // Iterate through the string
        for (i, c) in s.chars().enumerate() {
            cnt += 1;

            // Check if the current character is the last one or differs from the next character
            if i == n - 1 || c != s.chars().nth(i + 1).unwrap() {
                if cnt == k {
                    return true; // Found a special substring of length `k`
                }
                cnt = 0; // Reset the counter
            }
        }

        false // No special substring of length `k` found
    }
}

fn main() {
    // Set up input and output handling
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input
    let mut parts = input.trim().split_whitespace();
    let s = parts.next().expect("Expected a string").to_string();
    let k: usize = parts
        .next()
        .expect("Expected an integer")
        .parse()
        .expect("Failed to parse integer");

    // Create a solution instance and call the function
    let sol = Solution;
    let result = sol.has_special_substring(&s, k);

    // Output the result
    println!("{}", result);
}