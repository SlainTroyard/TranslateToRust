// Translated from C++ to Rust, preserving the logic and I/O format
use std::cmp;
use std::io::{self, BufRead};

/// Computes the maximum possible score given `start` and `d`.
fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort the input array
    start.sort();

    // The check function verifies if a given score is feasible
    let check = |score: i64| {
        let mut x = i64::MIN;
        for &s in start.iter() {
            // We place x as far right as possible while respecting the score spacing
            x = cmp::max(x + score, s as i64);
            // If x exceeds the allowed range [s, s + d], then it's not feasible
            if x > s as i64 + d as i64 {
                return false;
            }
        }
        true
    };

    // Establish binary search boundaries
    let mut left = 0;
    let mut right = ((start[start.len() - 1] + d) - start[0]) / ((start.len() - 1) as i32) + 1;

    // Binary search to find the maximum feasible score
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid as i64) {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read startSize and d
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let start_size: usize = parts.next().unwrap().parse().unwrap();
    let d: i32 = parts.next().unwrap().parse().unwrap();

    // Read the array `start`, which has `start_size` elements
    let mut start = Vec::with_capacity(start_size);
    let mut count = 0;
    while count < start_size {
        if let Some(line) = lines.next() {
            let line_str = line?;
            for num_str in line_str.split_whitespace() {
                if count < start_size {
                    let val: i32 = num_str.parse().unwrap();
                    start.push(val);
                    count += 1;
                } else {
                    break;
                }
            }
        } else {
            break;
        }
    }

    // Compute and print the result
    let result = max_possible_score(&mut start, d);
    println!("{}", result);

    Ok(())
}