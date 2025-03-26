// Problem: Weekly Contest 426 Problem 1
use std::io;

struct Solution;

impl Solution {
    // Function to calculate the smallest number
    pub fn smallest_number(n: i32) -> i32 {
        let b = (n as f64).log2().ceil() as i32 + 1; // Calculate the number of bits
        (1 << b) - 1                                // Return 2^b - 1
    }
}

fn main() {
    let mut input = String::new();

    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input as an integer
    let n: i32 = input.trim().parse().expect("Invalid input");

    // Create a solution instance
    let solution = Solution;

    // Calculate the smallest number
    let result = solution.smallest_number(n);

    // Output the result to stdout
    println!("{}", result);
}