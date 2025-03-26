// Problem: Weekly Contest 417 Problem 1
use std::io;

struct Solution;

impl Solution {
    fn kth_character(k: i64) -> char {
        // __builtin_popcount counts the number of 1 bits in the binary representation
        // In Rust, we can use count_ones() method
        let count = (k - 1).count_ones() as u8;
        (b'a' + count) as char
    }
}

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Parse input as i64
    let k: i64 = input.trim().parse().expect("Input should be a number");
    
    // Create solution instance and call the method
    let s = Solution;
    println!("{}", s.kth_character(k));
}