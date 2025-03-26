use std::io::{self, Read};

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort the array in ascending order
    start.sort_unstable();

    let start_size = start.len();
    if start_size == 0 {
        return 0; // Handle empty input, though problem constraints likely prevent this
    }

    // Calculate the initial right bound for binary search
    let min_val = start[0];
    let max_val = start[start_size - 1];
    let denominator = (start_size - 1) as i32;
    let right = if denominator == 0 {
        // Handle start_size=1 case (original code would divide by zero)
        // This case isn't properly handled in C code but included here for safety
        max_val + d + 1
    } else {
        (max_val + d - min_val) / denominator + 1
    };

    let mut left = 0;
    let mut right = right as i32;

    // Binary search to find maximum valid interval
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        let mut sum = i64::MIN;
        let mut valid = true;

        // Check if current mid satisfies the constraints
        for &elem in start.iter() {
            sum = (sum + mid as i64).max(elem as i64);
            if sum > (elem as i64 + d as i64) {
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
    // Read all input and parse into integers
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input format"))
        .collect();

    // Extract parameters and array elements
    let mut nums = numbers.into_iter();
    let start_size = nums.next().expect("Missing start_size") as usize;
    let d = nums.next().expect("Missing d");
    let mut start: Vec<i32> = nums.by_ref().take(start_size).collect();

    // Ensure exactly start_size elements were read
    if start.len() != start_size {
        panic!("Insufficient elements for start array");
    }

    // Compute and print the result
    println!("{}", max_possible_score(&mut start, d));
}