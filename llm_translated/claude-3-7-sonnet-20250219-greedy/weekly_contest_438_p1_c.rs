use std::io::{self, BufRead};

/// Checks if a string has the same digits after applying a specific transformation.
///
/// The transformation works by:
/// 1. Converting each character to its numeric value (by masking with 0x0f)
/// 2. Repeatedly replacing each pair of adjacent digits with their sum modulo 10
/// 3. Checking if the final two digits are the same
fn has_same_digits(s: &mut String) -> bool {
    let mut chars: Vec<u8> = s.bytes().collect();
    let mut len = chars.len();
    
    // Convert each character to its numeric value
    for i in 0..len {
        chars[i] &= 0x0f;
    }
    
    // Apply the transformation
    while len > 1 {
        len -= 1;
        for i in 0..len {
            chars[i] = (chars[i] + chars[i + 1]) % 10;
        }
    }
    
    // Check if the final two digits are the same
    chars[0] == chars[1]
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    
    // Read a line from stdin
    stdin.lock().read_line(&mut line).expect("Failed to read line");
    
    // Remove trailing newline
    line = line.trim().to_string();
    
    // Call the function and print the result
    let result = has_same_digits(&mut line);
    println!("{}", if result { 1 } else { 0 });
}