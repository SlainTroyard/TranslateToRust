// This Rust program solves the LeetCode Weekly Contest 425 Problem 2.
// It reads two strings (s and t) and an integer k from standard input,
// then checks if it is possible to rearrange s into t by partitioning them into
// equal sized chunks (each of size n/k), counting the occurrences of each chunk.
//
// The program preserves the input/output format of the original C++ code.

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead, Write};

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let n = s.len();
    // Each chunk should be of equal length (n / k)
    let chunk_size = n / k;
    
    let mut count_map = HashMap::new();
    
    // Increase count for each chunk in s
    for i in (0..n).step_by(chunk_size) {
        // Extract the substring from s with length chunk_size
        let chunk = &s[i..i + chunk_size];
        *count_map.entry(chunk.to_string()).or_insert(0) += 1;
    }
    
    // Decrease count for each chunk in t
    for i in (0..n).step_by(chunk_size) {
        let chunk = &t[i..i + chunk_size];
        *count_map.entry(chunk.to_string()).or_insert(0) -= 1;
    }
    
    // Check if all counts are zero
    count_map.values().all(|&v| v == 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from standard input into a string
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read s (first token/line input)
    let s = match lines.next() {
        Some(line) => line?,
        None => return Err("No input for s".into()),
    };

    // Read t (second token/line input)
    let t = match lines.next() {
        Some(line) => line?,
        None => return Err("No input for t".into()),
    };

    // Read k (third token/line input)
    let k_line = match lines.next() {
        Some(line) => line?,
        None => return Err("No input for k".into()),
    };
    // Parse k as usize (k is used in division so we use usize)
    let k: usize = k_line.trim().parse()?;
    
    // Call the function with the provided inputs
    let result = is_possible_to_rearrange(&s, &t, k);

    // Print the result exactly as "true" or "false"
    // Using stdout lock for efficient printing.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if result {
        writeln!(handle, "true")?;
    } else {
        writeln!(handle, "false")?;
    }
    
    Ok(())
}