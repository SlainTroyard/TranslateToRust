// Problem: Weekly Contest 422 Problem 1
// Translated from C to Rust with idiomatic usage and proper error handling.
// The program reads a single token from stdin (like scanf("%s", ...) would),
// checks if it's "balanced," and prints "true" or "false" to stdout.

use std::io::{self, BufRead};

/// Checks if the given string (representing a number) is "balanced"
/// according to the original C logic:
/// 
/// int total = 0;
/// while (*num) {
///     total += '0' - *num;
///     total = -total;
///     ++num;
/// }
/// return total == 0;
fn is_balanced(num: &str) -> bool {
    let mut total = 0i32;
    for c in num.chars() {
        total += ('0' as i32) - (c as i32);
        total = -total;
    }
    total == 0
}

fn main() -> io::Result<()> {
    // Read one line from stdin, then only consider the first token (split by whitespace).
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;

    // Mimic scanf("%s") by splitting on whitespace and taking the first piece.
    let token = line.split_whitespace().next().unwrap_or("");

    // Check if the number is balanced and print the result
    if is_balanced(token) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}