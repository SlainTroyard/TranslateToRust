use std::cmp;
use std::io::{self, BufRead};

/// Computes the maximum possible score based on the rules.
/// This function implements a binary search on the potential score increment.
/// It sorts the input array and then checks for feasibility of each candidate score.
fn max_possible_score(mut start: Vec<i64>, d: i64) -> i64 {
    let start_size = start.len();

    // Sort the array. Using Rust's built-in sort which is idiomatic.
    start.sort();

    // Set up binary search boundaries.
    // 'left' initially is 0.
    // 'right' is computed from the maximum possible average gap.
    let mut left: i64 = 0;
    let mut right: i64 = (start[start_size - 1] + d - start[0]) / ((start_size - 1) as i64) + 1;

    // Binary search loop:
    while left + 1 < right {
        let mid = left + (right - left) / 2;

        // Check if 'mid' is a valid score increment.
        // We simulate by tracking the cumulative sum.
        let mut sum: i64 = std::i64::MIN;
        let mut valid = true;
        for &val in &start {
            // Update the cumulative sum: either add mid to previous sum or restart at current value.
            sum = cmp::max(sum + mid, val);
            // If the cumulative sum exceeds the allowed maximum at this position, break early.
            if sum > val + d {
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
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read the first line from input.
    // The first line should contain two integers: startSize and d.
    let first_line = iterator
        .next()
        .expect("No input provided")
        .expect("Failed to read line");
    let mut first_iter = first_line.split_whitespace();

    // Parse startSize (number of elements in the array).
    let start_size = first_iter
        .next()
        .expect("No startSize value provided")
        .parse::<usize>()
        .expect("Failed to parse startSize");

    // Parse d.
    let d = first_iter
        .next()
        .expect("No d value provided")
        .parse::<i64>()
        .expect("Failed to parse d");

    // Prepare to read the array elements.
    let mut start_vec = Vec::with_capacity(start_size);

    // Process any additional numbers on the first line if available.
    for word in first_iter {
        if start_vec.len() < start_size {
            start_vec.push(
                word.parse::<i64>()
                    .expect("Failed to parse an integer in the array"),
            );
        }
    }

    // Read further lines until we have exactly start_size integers.
    while start_vec.len() < start_size {
        let line = iterator
            .next()
            .expect("Not enough lines for array input")
            .expect("Failed to read line");
        for word in line.split_whitespace() {
            if start_vec.len() < start_size {
                start_vec.push(
                    word.parse::<i64>()
                        .expect("Failed to parse an integer in the array"),
                );
            }
        }
    }

    // Compute the result using the max_possible_score function.
    let result = max_possible_score(start_vec, d);

    // Print the result to stdout.
    println!("{}", result);
}