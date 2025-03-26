// Weekly Contest 438 Problem 1 in Rust

use std::io::{self, BufRead};

// Define a struct to mirror the C++ class
struct Solution;

impl Solution {
    // Translate the hasSameDigits method
    // Takes a String s and returns a bool indicating if the final two digits match
    fn has_same_digits(&self, mut s: String) -> bool {
        // Repeat until only 2 characters remain
        while s.len() > 2 {
            let mut t = String::new();
            // Pairwise sum of adjacent digits
            for i in 0..(s.len() - 1) {
                let x = s.chars().nth(i).unwrap().to_digit(10).unwrap();
                let y = s.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                let sum = (x + y) % 10;
                t.push(std::char::from_digit(sum, 10).unwrap());
            }
            s = t;
        }
        // Compare the remaining two characters
        s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap()
    }
}

fn main() -> io::Result<()> {
    // Read a single string from stdin (like "cin >> s" in C++)
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input)?;
    let s = input.trim().to_string();

    // Create a Solution object and compute the result
    let sol = Solution;
    let result = sol.has_same_digits(s);

    // Print 1 if true, 0 if false (matching C++ "cout << bool")
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}