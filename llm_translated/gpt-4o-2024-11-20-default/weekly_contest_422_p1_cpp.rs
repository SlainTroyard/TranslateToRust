// Problem: Weekly Contest 422 Problem 1 in Rust
use std::io;

struct Solution;

impl Solution {
    fn is_balanced(num: &str) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        // Iterate through the digits of the number string
        for (i, ch) in num.chars().enumerate() {
            // Safely convert char to its numeric value
            let digit = ch.to_digit(10).expect("Input should only contain digits");
            
            // Accumulate based on even/odd position
            if i % 2 == 0 {
                even_sum += digit;
            } else {
                odd_sum += digit;
            }
        }

        // Check if the sums are equal
        even_sum == odd_sum
    }
}

fn main() {
    let mut input = String::new();

    // Read the input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Trim the input to remove any leading/trailing whitespace
    let num = input.trim();

    // Create a Solution instance and call the `is_balanced` method
    let solution = Solution;
    let result = solution.is_balanced(num);

    // Output the result as "true" or "false"
    if result {
        println!("true");
    } else {
        println!("false");
    }
}