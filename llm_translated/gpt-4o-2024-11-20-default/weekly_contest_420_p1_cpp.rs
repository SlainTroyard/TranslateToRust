// Problem: Weekly Contest 420 Problem 1
// Translated from C++ to Rust

use std::io::{self, Write};

struct Solution;

impl Solution {
    // Function implementing the string sequence logic
    pub fn string_sequence(target: &str) -> Vec<String> {
        let mut ans = Vec::new(); // Vector to store the resulting strings
        let mut s = String::new(); // Current string being built

        for c in target.chars() {
            s.push('a'); // Placeholder character
            for j in ('a'..=c) {
                s.pop();    // Remove last character
                s.push(j);  // Add the current character 'j'
                ans.push(s.clone()); // Add the current string `s` to the result vector
            }
        }

        ans
    }
}

fn main() {
    // Reading input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target = input.trim(); // Remove any trailing whitespace

    // Create an instance of the solution and get the result
    let ans = Solution::string_sequence(target);

    // Print the result, separating strings with a space
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for s in ans {
        write!(handle, "{} ", s).expect("Failed to write output");
    }
    writeln!(handle).expect("Failed to write final newline");
}