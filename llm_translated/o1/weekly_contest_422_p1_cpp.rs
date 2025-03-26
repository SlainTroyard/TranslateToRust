// Problem: Weekly Contest 422 Problem 1

// We will read a single line from stdin, interpret it as a string,
// and determine if the sum of the digits at even indices equals
// the sum of the digits at odd indices, exactly as the original C++ code
// did with cin >> num.

use std::io;

struct Solution;

impl Solution {
    // Checks if the string's even-indexed digits sum up to the same value
    // as the odd-indexed digits.
    fn is_balanced(&self, num: &str) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        // Enumerate to track the index (i) and character (c) simultaneously.
        for (i, c) in num.chars().enumerate() {
            // Convert the character to a digit. Unwrap is used here because
            // the original code assumes valid digits without explicit error handling.
            let digit = c.to_digit(10).unwrap();
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
    // Read the input number as a single string from stdin
    let mut num_input = String::new();
    io::stdin()
        .read_line(&mut num_input)
        .expect("Failed to read line");

    // Trim any trailing newline or whitespace
    let num = num_input.trim();

    // Create a Solution object and call the is_balanced function
    let solution = Solution;
    let result = solution.is_balanced(num);

    // Output the result in the exact same format as the original code
    if result {
        println!("true");
    } else {
        println!("false");
    }
}