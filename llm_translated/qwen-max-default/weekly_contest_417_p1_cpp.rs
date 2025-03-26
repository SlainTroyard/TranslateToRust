// Problem: Weekly Contest 417 Problem 1
use std::io;

struct Solution;

impl Solution {
    // This function calculates the k-th character in a specific sequence.
    // The sequence is such that the k-th character is 'a' + the number of 1-bits in the binary representation of (k-1).
    pub fn kth_character(k: u64) -> char {
        'a' + (k - 1).count_ones() as u8
    }
}

fn main() {
    // Read an integer from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: u64 = input.trim().parse().expect("Please type a number!");

    // Create an instance of the Solution struct and call the kth_character method
    let s = Solution;
    let result = s.kth_character(k);

    // Print the result to standard output
    println!("{}", result);
}