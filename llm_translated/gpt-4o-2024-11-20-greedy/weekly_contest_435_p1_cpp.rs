// Problem: Weekly Contest 435 Problem 1
use std::io::{self, Write};
use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_difference(s: &str) -> i32 {
        // Array to count occurrences of each character ('a' to 'z')
        let mut cnt = [0; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        let mut max1 = 0; // Maximum odd frequency
        let mut min0 = i32::MAX; // Minimum even frequency

        for &c in &cnt {
            if c % 2 == 1 {
                max1 = max(max1, c);
            } else if c > 0 {
                min0 = min(min0, c);
            }
        }

        max1 - min0
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Remove any trailing newline or whitespace

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.max_difference(s);

    // Print the result to stdout
    println!("{}", result);
}