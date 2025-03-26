// Weekly Contest 437 Problem 1 in Rust

use std::io;

/// Checks if a string contains a run of exactly `k` consecutive identical characters.
fn has_special_substring(s: &str, k: usize) -> bool {
    let bytes = s.as_bytes();
    let length = bytes.len();
    let mut consecutive = 0;

    for i in 0..length {
        consecutive += 1;
        // If we're at the end OR the next character differs from the current one
        if i == length - 1 || bytes[i] != bytes[i + 1] {
            // Check if the run length matches k
            if consecutive == k {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    // We expect input in the format "<string> <integer>"
    // e.g. "abc 3"
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // Read one line from stdin
    
    // Split the line by whitespace
    let mut parts = input.trim().split_whitespace();
    
    // Extract the string s
    let s = match parts.next() {
        Some(str_val) => str_val.to_string(),
        None => {
            // If we cannot parse the string, we mimic the C behavior
            // but in idiomatic Rust we handle the error gracefully
            eprintln!("Failed to read string.");
            return Ok(());
        }
    };
    
    // Extract the integer k
    let k: usize = match parts.next() {
        Some(num_val) => match num_val.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Failed to parse integer k.");
                return Ok(());
            }
        },
        None => {
            // If we cannot parse k, we mimic the C behavior
            eprintln!("Failed to read integer k.");
            return Ok(());
        }
    };
    
    // Print 1 if the function returns true, otherwise 0
    println!("{}", if has_special_substring(&s, k) { 1 } else { 0 });
    
    Ok(())
}