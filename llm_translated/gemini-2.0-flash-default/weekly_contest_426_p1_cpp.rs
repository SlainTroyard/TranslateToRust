// Problem: Weekly Contest 426 Problem 1
use std::io;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;

struct Solution {}

impl Solution {
    pub fn smallest_number(&self, n: i32) -> i32 {
        let b = (n as f64).log2() as i32 + 1; // Calculate the number of bits
        (1 << b) - 1 // Return 2^b - 1
    }
}

fn main() {
    let solution = Solution {};
    let mut n_str = String::new();

    // Input
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Please type a number!");

    // Calculate the smallest number
    let result = solution.smallest_number(n);

    // Output
    println!("{}", result);
}