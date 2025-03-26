// Problem: Weekly Contest 438 Problem 1
use std::io::{self, Write};

struct Solution;

impl Solution {
    // Function to check if the string reduces to two identical digits
    pub fn has_same_digits(mut s: String) -> bool {
        while s.len() > 2 {
            let mut t = String::new();
            let bytes = s.as_bytes();
            for i in 0..s.len() - 1 {
                let x = (bytes[i] - b'0') as u8;
                let y = (bytes[i + 1] - b'0') as u8;
                t.push(((x + y) % 10 + b'0') as char);
            }
            s = t;
        }
        s.chars().nth(0) == s.chars().nth(1)
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim().to_string();

    // Create an instance of the solution and call the function
    let sol = Solution;
    let result = sol.has_same_digits(s);

    // Print the result to stdout
    println!("{}", result);
}