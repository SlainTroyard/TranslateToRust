// Problem: Weekly Contest 426 Problem 1
use std::io;

struct Solution;

impl Solution {
    fn smallest_number(n: i32) -> i32 {
        let b = (n as f64).log2().floor() as i32 + 1;  // Calculate the number of bits
        (1 << b) - 1  // Return 2^b - 1
    }
}

fn main() {
    // Create a string to hold the input
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Parse the input as an integer
    let n: i32 = input
        .trim()
        .parse()
        .expect("Failed to parse input as integer");
    
    // Calculate the smallest number
    let solution = Solution;
    let result = solution.smallest_number(n);
    
    // Output the result
    println!("{}", result);
}