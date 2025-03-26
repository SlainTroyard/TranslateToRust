use std::io::{self, BufRead};

// Problem: Weekly Contest 417 Problem 4
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and operationSize
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let k: i64 = iter.next().unwrap().parse().unwrap();
    let operation_size: usize = iter.next().unwrap().parse().unwrap();

    // Read operations
    let second_line = lines.next().unwrap()?;
    let operations: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate and print the result
    let result = Solution::kth_character(k, &operations);
    println!("{}", result);

    Ok(())
}

struct Solution;

impl Solution {
    pub fn kth_character(k: i64, operations: &[i32]) -> char {
        // Convert k to 0-indexed
        let mut k = k - 1;
        let mut inc = 0;

        // Calculate the number of bits in k
        let mut i = 63;
        while i >= 0 {
            if k & (1 << i) != 0 {
                inc += operations[i as usize];
            }
            i -= 1;
        }

        // Calculate the final character
        (b'a' + (inc % 26) as u8) as char
    }
}