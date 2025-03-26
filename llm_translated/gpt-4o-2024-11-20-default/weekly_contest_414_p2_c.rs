use std::io::{self, Read};
use std::cmp::max;

// Function to implement the original algorithm
fn max_possible_score(start: &mut Vec<i32>, d: i32) -> i32 {
    // Step 1: Sort the array
    start.sort_unstable();

    // Step 2: Binary search to find the maximum possible score
    let mut left = 0;
    let mut right = (start[start.len() - 1] + d - start[0]) / (start.len() as i32 - 1) + 1;

    while left + 1 < right {
        let mid = left + (right - left) / 2;

        // Check if the mid value is valid according to the condition
        let mut sum: i64 = i64::MIN;
        let mut valid = true;
        for &value in start.iter() {
            sum = max(sum + mid as i64, value as i64);
            if sum > (value + d) as i64 {
                valid = false;
                break;
            }
        }

        if valid {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    // Parse input values
    let mut lines = input.lines();
    let first_line = lines.next().expect("Expected input lines");
    let mut first_iter = first_line.split_whitespace();
    let start_size: usize = first_iter.next()
        .and_then(|s| s.parse().ok())
        .expect("Expected integer for start size");
    let d: i32 = first_iter.next()
        .and_then(|s| s.parse().ok())
        .expect("Expected integer for d");
    
    let second_line = lines.next().expect("Expected input lines");
    let mut start: Vec<i32> = second_line.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Expected integer in start array"))
        .collect();

    assert_eq!(start.len(), start_size, "Size of start array doesn't match the given start_size");

    // Call the function and print the result
    let result = max_possible_score(&mut start, d);
    println!("{}", result);
}