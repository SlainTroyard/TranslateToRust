// Problem: Weekly Contest 417 Problem 4
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: &[i32]) -> char {
        let mut k = k - 1; // Decrement k to make it 0-based
        let mut inc = 0;

        // Iterate over the bits of k from the most significant bit to the least
        for i in (0..=k.ilog2()).rev() {
            if (k >> i) & 1 == 1 {
                inc += operations[i as usize];
            }
        }

        // Calculate the resulting character
        (b'a' + (inc % 26) as u8) as char
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k
    let k: i64 = lines
        .next()
        .expect("Expected input for k")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse k as i64");

    // Read the size of the operations array
    let operation_size: usize = lines
        .next()
        .expect("Expected input for operation size")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse operation size as usize");

    // Read the operations array
    let operations: Vec<i32> = lines
        .next()
        .expect("Expected input for operations array")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse operation as i32"))
        .collect();

    // Ensure the operations array has the correct size
    assert_eq!(
        operations.len(),
        operation_size,
        "The number of operations does not match the specified size"
    );

    // Solve the problem
    let solution = Solution;
    let result = solution.kth_character(k, &operations);

    // Print the result
    println!("{}", result);
}