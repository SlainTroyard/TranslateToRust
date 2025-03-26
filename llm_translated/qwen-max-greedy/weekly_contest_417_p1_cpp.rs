// Problem: Weekly Contest 417 Problem 1

use std::io;

struct Solution;

impl Solution {
    // Function to determine the k-th character in the sequence
    fn kth_character(k: u64) -> char {
        // Calculate the number of 1-bits in the binary representation of (k - 1)
        // and use it to determine the character
        'a' + (k - 1).count_ones() as u8
    }
}

fn main() {
    // Read the input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input to an integer
    let k: u64 = input.trim().parse().expect("Please type a number!");

    // Create an instance of the Solution struct
    let s = Solution;

    // Call the kth_character method and print the result to stdout
    println!("{}", s.kth_character(k));
}