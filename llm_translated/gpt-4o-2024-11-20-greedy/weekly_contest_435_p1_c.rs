// Problem: Weekly Contest 435 Problem 1 in Rust
use std::io::{self, Read};

/// Function to calculate max difference between the largest odd frequency
/// and smallest non-zero even frequency of characters in a string.
fn max_difference(s: &str) -> i32 {
    let mut freq = [0; 26]; // Array to store frequencies of characters (a-z)
    
    // Count character frequencies
    for ch in s.bytes() {
        freq[(ch - b'a') as usize] += 1;
    }

    let mut max_odd = 0; // Maximum odd frequency
    let mut min_even = i32::MAX; // Minimum non-zero even frequency

    for &count in &freq {
        // If the count is odd and greater than current max_odd, update it
        if count % 2 == 1 {
            max_odd = i32::max(max_odd, count);
        }
        // If the count is a non-zero even number and less than min_even, update it
        else if count != 0 {
            min_even = i32::min(min_even, count);
        }
    }

    // Calculate and return the difference between the largest odd and smallest non-zero even
    max_odd - min_even
}

fn main() {
    // Read input string from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Trim input to remove any trailing newlines or extra spaces
    let input = input.trim();

    // Call the max_difference function and print the result
    println!("{}", max_difference(input));
}