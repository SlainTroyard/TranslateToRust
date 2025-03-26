use std::cmp;
use std::error::Error;
use std::io::{self, BufRead, Read};

/// Computes the maximum possible score according to the given logic.
///
/// The function sorts the start vector and uses a binary search
/// to determine the largest score that meets the condition enforced
/// by the `check` closure.
///
/// The closure simulates the scoring conditions: starting from the smallest
/// element, it ensures that the incremented score never exceeds (s + d) for each s.
fn max_possible_score(mut start: Vec<i64>, d: i64) -> i64 {
    // Sort the vector to simulate the same logic as C++ sort.
    start.sort();
    let n = start.len();

    // Define a closure to check if a given score is valid.
    let check = |score: i64| -> bool {
        // Start with the minimum possible i64 value (similar to LLONG_MIN in C++).
        let mut x: i64 = i64::MIN;
        for &s in &start {
            // Compute the maximum between x + score and s.
            x = cmp::max(x + score, s);
            // If x exceeds the allowed limit (s + d) then the score is not valid.
            if x > s + d {
                return false;
            }
        }
        true
    };

    // Initialize binary search boundaries.
    let mut left: i64 = 0;
    // Calculate an upper bound for score: (last element + d - first element) / (n - 1) + 1.
    let right_bound = (start[n - 1] + d - start[0]) / ((n - 1) as i64) + 1;
    let mut right = right_bound;

    // Perform binary search: find the largest score for which `check` is true.
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn main() -> Result<(), Box<dyn Error>> {
    // Prepare a buffered reader for standard input.
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut input = String::new();
    // Read the entire input into a string.
    reader.read_to_string(&mut input)?;

    // Process input tokens split by whitespace,
    // mimicking the C++ extraction operator (>>) in the original code.
    let mut tokens = input.split_whitespace();

    // Read the first two values: startSize and d.
    let start_size: usize = tokens
        .next()
        .ok_or("Missing startSize")?
        .parse()
        .map_err(|_| "Failed parsing startSize")?;
    let d: i64 = tokens
        .next()
        .ok_or("Missing d")?
        .parse()
        .map_err(|_| "Failed parsing d")?;

    // Read the start vector elements.
    let mut start = Vec::with_capacity(start_size);
    for _ in 0..start_size {
        let value: i64 = tokens
            .next()
            .ok_or("Missing vector value")?
            .parse()
            .map_err(|_| "Failed parsing a vector value")?;
        start.push(value);
    }

    // Calculate the maximum possible score.
    let result = max_possible_score(start, d);

    // Output the result in the exact format expected.
    println!("{}", result);

    Ok(())
}