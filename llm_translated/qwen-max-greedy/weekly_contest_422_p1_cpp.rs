// Problem: Weekly Contest 422 Problem 1

use std::io;

struct Solution;

impl Solution {
    /// Checks if the sum of digits at even indices is equal to the sum of digits at odd indices.
    fn is_balanced(num: &str) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        for (i, c) in num.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as i32;
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
    // Read the input number as a string
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");

    // Trim the newline character and any trailing whitespace
    num = num.trim().to_string();

    // Create a Solution object and call the is_balanced function
    let solution = Solution;
    let result = Solution::is_balanced(&num);

    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}