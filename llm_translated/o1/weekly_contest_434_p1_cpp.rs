// Translation of C++ code for LeetCode Weekly Contest 434 Problem 1 into Rust.
//
// The original C++ solution reads an integer n, then reads n integers into a vector,
// computes a result, and prints it. Exact same I/O format (stdin/stdout) is preserved.
//
// USAGE: 
//     1) First line of input: integer n
//     2) Followed by n integers either on the same line or split across multiple lines
//     3) Prints one integer as the result

use std::io::{self, BufRead};

fn main() {
    // Read the integer n
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input for n");
    let n: usize = input.trim().parse().expect("Failed to parse n");

    // Read n integers from stdin
    let mut nums = Vec::with_capacity(n);
    while nums.len() < n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line of values");
        for token in line.split_whitespace() {
            let val: i32 = token.parse().expect("Failed to parse an integer");
            nums.push(val);
            if nums.len() == n {
                break;
            }
        }
    }

    // Compute the result using the same logic as the original C++ code
    let ans = count_partitions(&nums);

    // Print the result (exactly one integer)
    println!("{}", ans);
}

// Translated "Solution::countPartitions" logic
fn count_partitions(nums: &[i32]) -> i32 {
    let s: i32 = nums.iter().sum();
    if s % 2 != 0 {
        0
    } else {
        nums.len() as i32 - 1
    }
}