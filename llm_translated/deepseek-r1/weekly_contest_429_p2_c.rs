use std::io::{self, Read};
use std::i32;

fn main() {
    // Read all input at once and split into whitespace-separated tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().flat_map(|s| s.parse::<i32>());

    // Parse n (array size) and diff from input
    let n = tokens.next().expect("Missing array size");
    let diff = tokens.next().expect("Missing difference value");
    
    // Collect exactly n elements for the array
    let mut arr: Vec<i32> = tokens.take(n as usize).collect();
    if arr.len() != n as usize {
        panic!("Not enough elements in the array");
    }

    // Calculate and print the result
    let result = max_distinct_elements(&mut arr, diff);
    println!("{}", result);
}

/// Finds maximum distinct elements after allowing +/- diff adjustments
fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    // Sort array first for greedy processing
    arr.sort_unstable();

    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    for &num in arr.iter() {
        // Determine the smallest possible value we can assign to this element
        let x = (prev + 1).max(num - diff);

        // Check if this value is within the allowed range
        if x <= num + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}