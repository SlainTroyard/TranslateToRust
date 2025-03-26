// Problem: Weekly Contest 422 Problem 1

use std::io::{self, BufRead};

/// Checks if the given number string is balanced.
///
/// A number is considered balanced if the sum of its digits,
/// when alternately subtracted and added, equals zero.
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for c in num.chars() {
        total += '0' as i32 - c as i32;
        total = -total;
    }
    total == 0
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number as a string
    let num = lines.next().unwrap()?.trim().to_string();

    // Check if the number is balanced
    if is_balanced(&num) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}