// Problem: Weekly Contest 437 Problem 1

use std::io;

fn has_special_substring(s: &str, k: usize) -> bool {
    let length = s.len();
    let mut consecutive = 0;
    for (i, c) in s.chars().enumerate() {
        consecutive += 1;
        if i == length - 1 || c != s.chars().nth(i + 1).unwrap_or(' ') {
            if consecutive == k {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Parse the input
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Invalid input format");
        return Ok(());
    }

    let s = parts[0];
    let k: usize = parts[1].parse()?;

    // Call the function and print the result
    let result = has_special_substring(s, k);
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}