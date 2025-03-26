// Problem: Weekly Contest 435 Problem 2
use std::io::{self, Write}; // For input/output handling use idiomatic std::io
use std::cmp::{min, max};  // For min and max functions

struct Solution;

impl Solution {
    // Translated function `maxDistance` from C++
    pub fn max_distance(s: &str, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        
        for (i, ch) in s.chars().enumerate() {
            match ch {
                'N' => y += 1,  // Move up
                'S' => y -= 1,  // Move down
                'E' => x += 1,  // Move right
                'W' => x -= 1,  // Move left
                _ => unreachable!(), // Handle unexpected characters
            }
            let current_distance = (x.abs() + y.abs()) as i32 + k * 2;  // Calculate max distance possible
            ans = max(ans, min(current_distance, (i + 1) as i32)); // Update answer
        }
        ans
    }
}

fn main() {
    // Reading input: A string `s` and an integer `k` from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    // Parse input into components
    let mut input_parts = input.split_whitespace();
    let s = input_parts.next().expect("Missing string input");
    let k: i32 = input_parts.next().expect("Missing integer input").parse().expect("Invalid k input");

    // Create the solution object and calculate the answer
    let sol = Solution;
    let result = sol.max_distance(s, k);

    // Output the result
    let stdout = io::stdout(); 
    let mut stdout_lock = stdout.lock();
    writeln!(stdout_lock, "{}", result).expect("Failed to write output");
}