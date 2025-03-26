// Problem: Weekly Contest 422 Problem 1
use std::io::{self, Write};

struct Solution;

impl Solution {
    // Function to check if the number is balanced
    pub fn is_balanced(num: &str) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        for (i, ch) in num.chars().enumerate() {
            // Convert character to digit
            let digit = ch.to_digit(10).expect("Input should only contain digits") as i32;

            // Add to even_sum or odd_sum based on index parity
            if i % 2 == 0 {
                even_sum += digit;
            } else {
                odd_sum += digit;
            }
        }

        even_sum == odd_sum
    }
}

fn main() {
    // Create a buffer for input
    let mut input = String::new();

    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Trim the input to remove any trailing newline or whitespace
    let num = input.trim();

    // Create a Solution object and call the is_balanced function
    let solution = Solution;
    let result = solution.is_balanced(num);

    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}