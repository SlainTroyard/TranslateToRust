/// Translated from the original C code for LeetCode Weekly Contest 429 Problem 2.
/// The logic and I/O format are preserved exactly as in the C version.

use std::io::{self, BufRead};

/// Reads all tokens (split by any whitespace) from stdin
/// and returns an iterator of tokens as strings.
fn read_tokens() -> impl Iterator<Item = String> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .flat_map(|line| line.split_whitespace().map(str::to_string).collect::<Vec<_>>())
}

/// Returns the maximum number of distinct elements we can pick
/// respecting the condition that for each arr[i], we pick a value x
/// such that (arr[i] - diff) <= x <= (arr[i] + diff) and each picked x
/// is strictly greater than the previously chosen x.
fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    // Sort the array in ascending order
    arr.sort();

    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // For every element in the sorted array:
    // choose x = max(prev + 1, arr[i] - diff)
    // if x <= arr[i] + diff, increment the result and update prev
    for &val in arr {
        let x = (prev + 1).max(val - diff);
        if x <= val + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() {
    // We will read exactly the same inputs in the same order as the C code:
    // 1. Integer n (array size)
    // 2. Integer diff
    // 3. n integers for arr
    let mut tokens = read_tokens();
    
    // Read the array size
    let n = tokens
        .next()
        .expect("Expected an integer for n")
        .parse::<i32>()
        .expect("Failed to parse n as an integer");
    
    // Read the difference
    let diff = tokens
        .next()
        .expect("Expected an integer for diff")
        .parse::<i32>()
        .expect("Failed to parse diff as an integer");
    
    // Read the array elements
    let mut arr = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let val = tokens
            .next()
            .expect("Expected an integer for array element")
            .parse::<i32>()
            .expect("Failed to parse array element as integer");
        arr.push(val);
    }
    
    // Compute the result
    let result = max_distinct_elements(&mut arr, diff);
    
    // Output the result (matches the exact C code output format)
    println!("{}", result);
}