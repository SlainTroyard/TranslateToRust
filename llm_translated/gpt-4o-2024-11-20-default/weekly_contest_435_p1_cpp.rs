// Problem: Weekly Contest 435 Problem 1
use std::io;
use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut cnt = [0; 26];
        
        // Count occurrences of each character
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        let mut max1 = 0;
        let mut min0 = i32::MAX;

        // Iterate over frequencies in `cnt`
        for &c in cnt.iter() {
            if c % 2 != 0 {
                max1 = max(max1, c);
            } else if c > 0 { // Ensure non-zero counts for even frequencies
                min0 = min(min0, c);
            }
        }

        max1 - min0
    }
}

fn main() {
    // Setup for reading input from stdin and writing to stdout
    let mut input = String::new();
    let stdin = io::stdin();
    
    // Read the input string from stdin
    stdin.read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    // Create the solution object and calculate the result
    let sol = Solution;
    let result = sol.max_difference(s);

    // Print the result to stdout
    println!("{}", result);
}