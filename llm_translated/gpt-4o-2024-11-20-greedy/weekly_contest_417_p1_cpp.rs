// Problem: Weekly Contest 417 Problem 1
use std::io;

struct Solution;

impl Solution {
    // Function to compute the kth character
    fn kth_character(k: u64) -> char {
        // Calculate the number of 1 bits in (k - 1) and add it to 'a'
        ('a' as u8 + (k - 1).count_ones() as u8) as char
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input as an integer
    let k: u64 = input.trim().parse().expect("Invalid input");

    // Create an instance of Solution and compute the result
    let solution = Solution;
    let result = solution.kth_character(k);

    // Print the result to stdout
    println!("{}", result);
}