use std::io::{self, BufRead};
use std::cmp::Ordering;

fn max_distinct_elements(mut arr: Vec<i32>, diff: i32) -> i32 {
    let mut prev = i32::MIN;
    let mut distinct_count = 0;

    // Sort the array in ascending order
    arr.sort_unstable();

    for &value in &arr {
        // Calculate the smallest possible distinct value within the range
        let x = if prev + 1 > value - diff {
            prev + 1
        } else {
            value - diff
        };

        // Check if x is within the valid range
        if x <= value + diff {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the array
    let n: usize = lines
        .next()
        .expect("Expected input for array size")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse array size");

    // Read the difference value
    let diff: i32 = lines
        .next()
        .expect("Expected input for difference value")
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse difference value");

    // Read the array elements
    let arr: Vec<i32> = lines
        .next()
        .expect("Expected input for array elements")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse array element"))
        .collect();

    // Ensure the input array size matches the provided size
    assert_eq!(arr.len(), n, "Array size does not match the provided size");

    // Compute the result
    let result = max_distinct_elements(arr, diff);

    // Output the result
    println!("{}", result);
}