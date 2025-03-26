// Problem: Weekly Contest 426 Problem 1
use std::io::{self, BufRead};

struct Solution {}

impl Solution {
    fn smallest_number(n: i32) -> i32 {
        // Calculate the number of bits required to represent n
        let b = (n as f64).log2() as i32 + 1;
        
        // Return 2^b - 1
        (1 << b) - 1
    }
}

fn main() -> io::Result<()> {
    // Set up input reader
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input: Read n from stdin
    let n: i32 = lines
        .next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .parse()
        .expect("Failed to parse n as an integer");
    
    // Calculate the smallest number
    let result = Solution::smallest_number(n);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}