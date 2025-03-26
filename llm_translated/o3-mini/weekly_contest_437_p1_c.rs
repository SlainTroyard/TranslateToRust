use std::error::Error;
use std::io::{self, Read};

/// Checks if the given string has a special substring:
/// a segment of exactly k consecutive identical characters.
/// This function mimics the C logic from the original code.
fn has_special_substring(s: &str, k: usize) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut consecutive = 0;

    // Iterate over indices of the byte slice.
    for i in 0..len {
        consecutive += 1;
        // When we reach the end of a group (or end of string)
        if i == len - 1 || bytes[i] != bytes[i + 1] {
            if consecutive == k {
                return true;
            }
            // Reset counter if not exactly k
            consecutive = 0;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the entire input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split the input into whitespace-separated tokens.
    // This handles both single and multi-line input formats.
    let mut tokens = input.split_whitespace();

    // Extract the string token (like using %s in scanf)
    let s = tokens
        .next()
        .ok_or("Expected a string input for s, but not found")?;

    // Extract the integer token (like using %d in scanf) and convert it to usize.
    let k_str = tokens
        .next()
        .ok_or("Expected an integer input for k, but not found")?;
    let k = k_str.parse::<usize>()?;

    // Call the function to check for a special substring.
    let result = has_special_substring(s, k);

    // Print the result as 1 (true) or 0 (false) followed by a newline,
    // exactly matching the C code's output format.
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}