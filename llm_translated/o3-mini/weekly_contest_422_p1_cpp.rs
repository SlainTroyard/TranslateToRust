use std::io::{self, BufRead};

/// A structure representing our solution.
struct Solution;

impl Solution {
    /// Checks if the provided string representation of a number is balanced.
    /// A number is balanced if the sum of digits at even indices equals the sum of digits at odd indices.
    fn is_balanced(num: &str) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        // Iterate through each character (digit) with its index.
        for (i, ch) in num.chars().enumerate() {
            // Convert character to digit.
            // We use `to_digit(10)` which returns an Option<u32>. We expect all characters are digits.
            let digit = ch.to_digit(10).expect("Input string should contain only digits") as i32;
            if i % 2 == 0 {
                even_sum += digit;
            } else {
                odd_sum += digit;
            }
        }
        // Return true if the sums are equal.
        even_sum == odd_sum
    }
}

fn main() {
    // Create a new buffer for reading standard input.
    let stdin = io::stdin();
    // We assume input might be in a single line; hence, we capture the first non-empty token.
    let mut input = String::new();
    
    // Read one token from input. In C++ `cin >> num` reads a whitespace separated token.
    // Here we read one line and then extract the first token.
    if let Err(e) = stdin.lock().read_line(&mut input) {
        eprintln!("Failed to read from stdin: {}", e);
        return;
    }

    // Trim any whitespace from the input line.
    // We do not split the input further because the original C++ code reads one token.
    let num = input.trim();

    // Create a solution object and check if the number is balanced.
    let result = Solution::is_balanced(num);

    // Output the result, matching the original format.
    if result {
        println!("true");
    } else {
        println!("false");
    }
}