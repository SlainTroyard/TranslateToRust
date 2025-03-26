/// Translated from the C code of LeetCode Weekly Contest 435 Problem 1
/// This program reads a single string from stdin, computes the difference
/// between the maximum odd frequency and minimum even (non-zero) frequency
/// of characters, and prints the result.

use std::io::{self, BufRead};

/// Counts the frequency of each character in `s` (assuming `a` to `z` only),
/// finds the maximum frequency among characters with an odd count (`c1`)
/// and the minimum frequency among characters with a non-zero even count (`c2`),
/// then returns `c1 - c2`.
fn max_difference(s: &str) -> i32 {
    // We assume only 'a' to 'z' are considered, as in the original code.
    let mut cnt = [0; 26];
    for &b in s.as_bytes() {
        // Only count letters 'a' to 'z'.
        if b >= b'a' && b <= b'z' {
            cnt[(b - b'a') as usize] += 1;
        }
    }

    let mut c1 = 0;    // maximum odd frequency
    let mut c2 = 100;  // minimum even (non-zero) frequency

    for &frequency in &cnt {
        if frequency & 1 == 1 {
            // If odd, update c1 if this frequency is larger.
            if frequency > c1 {
                c1 = frequency;
            }
        } else if frequency != 0 {
            // If even (non-zero), update c2 if this frequency is smaller.
            if frequency < c2 {
                c2 = frequency;
            }
        }
    }

    c1 - c2
}

fn main() -> io::Result<()> {
    // Read one line from stdin.
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;

    // Extract the first token (equivalent to `scanf("%s", s)`).
    if let Some(s) = line.split_whitespace().next() {
        // Compute and print the answer, matching the output format in C.
        println!("{}", max_difference(s));
    } else {
        // If no input was provided, we mimic behavior (C code would be undefined).
        // Here, we simply do nothing or you could handle it more explicitly.
        // We'll end gracefully.
    }

    Ok(())
}