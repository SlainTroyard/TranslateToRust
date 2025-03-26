// Problem: Weekly Contest 428 Problem 1
//
// This Rust program reads input from standard input with the following format:
//   n
//   x₀ y₀
//   x₁ y₁
//   ...
//   xₙ₋₁ yₙ₋₁
//
// It computes the button with the longest time difference as described in the C++ version.
// The algorithm preserves the exact logic from the original C++ code.
// For idiomatic Rust error handling, we use the Result type in main and proper error propagation.

use std::error::Error;
use std::io::{self, BufRead};

/// Returns the button index with the longest time difference.
/// The function expects the input events to be a vector of vectors,
/// where each inner vector has two integers: [button index, timestamp].
fn button_with_longest_time(events: &[Vec<i32>]) -> i32 {
    // Initialize with the first event's button and its timestamp.
    // Note: As per original code, the first event's timestamp is used as the initial max_diff.
    let mut idx = events[0][0];
    let mut max_diff = events[0][1];

    // Compare differences between consecutive events.
    // p corresponds to events[i - 1] and q corresponds to events[i]
    for i in 1..events.len() {
        let previous = &events[i - 1];
        let current = &events[i];
        let d = current[1] - previous[1];
        // Update if current difference is greater, or if equal but button index is smaller.
        if d > max_diff || (d == max_diff && current[0] < idx) {
            idx = current[0];
            max_diff = d;
        }
    }

    idx
}

fn main() -> Result<(), Box<dyn Error>> {
    // Lock standard input for efficient reading.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of events.
    let n_line = lines.next().ok_or("Expected a line with number of events")??;
    let n: usize = n_line.trim().parse()?;

    // Read each event line, each should have exactly two integers.
    let mut events = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().ok_or("Expected more event lines")??;
        // Split the line on whitespace and parse the two integers.
        let tokens: Vec<i32> = line
            .split_whitespace()
            .map(str::trim)
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        if tokens.len() != 2 {
            return Err("Each event line must contain exactly 2 integers".into());
        }
        events.push(tokens);
    }

    // Solve the problem using the provided events.
    let result = button_with_longest_time(&events);
    
    // Print the result to stdout.
    println!("{}", result);

    Ok(())
}