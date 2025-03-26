/// Problem: Weekly Contest 429 Problem 1
/// Translated from C to Rust with the same logic and I/O format.
use std::io::{self, BufRead};

/// This function replicates the `minimumOperations` logic from the C code.
/// It scans the array from the end to the beginning, counting occurrences
/// of each value in a fixed-size array. Once a duplicate is found,
/// it returns (index + 3) / 3. If no duplicates are found, it returns 0.
fn minimum_operations(nums: &[i32]) -> i32 {
    let mut count = [0; 101]; // Count array for values from 0..100
    for (i, &val) in nums.iter().enumerate().rev() {
        let idx = val as usize;
        count[idx] += 1;
        if count[idx] > 1 {
            // Return (i + 3) / 3 as in the C code
            return ((i + 3) / 3) as i32;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all tokens from stdin (supports multiple lines/values per line)
    let stdin = io::stdin();
    let tokens: Vec<String> = stdin
        .lock()
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|w| w.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    // First token is the number of elements 'n'
    let n = tokens[0].parse::<usize>()?;

    // Next 'n' tokens are the array elements
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        nums.push(tokens[i + 1].parse::<i32>()?);
    }

    // Perform the operation and print the result (same as printf("%d\n", result);)
    let result = minimum_operations(&nums);
    println!("{}", result);

    Ok(())
}