// Problem: Weekly Contest 417 Problem 4
use std::io;
use std::io::{BufRead, Write};

struct Solution;

impl Solution {
    fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut k = k - 1; // Convert k to 0-based indexing
        let mut inc = 0;

        // Custom implementation of `__lg(k)` is `k.leading_zeros()`
        for i in (0..=63 - k.leading_zeros() as i32).rev() {
            // Check if the i-th bit is set in k
            if (k >> i) & 1 == 1 {
                inc += operations[i as usize];
            }
        }

        // Compute the resulting character as 'a' + (inc % 26)
        ('a' as u8 + (inc % 26) as u8) as char
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Read k and operation size
    stdin.lock().read_line(&mut input)?;
    let mut tokens = input.trim().split_whitespace();
    let k: i64 = tokens.next().unwrap().parse().unwrap();
    let operation_size: usize = tokens.next().unwrap().parse().unwrap();

    // Read the operation values
    input.clear();
    stdin.lock().read_line(&mut input)?;
    let operations: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the operation size matches the input data
    assert_eq!(operations.len(), operation_size);

    // Compute the result and print it
    let result = Solution::kth_character(k, operations);
    writeln!(stdout, "{}", result)?;

    Ok(())
}