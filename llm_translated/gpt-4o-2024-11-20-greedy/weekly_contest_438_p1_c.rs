use std::io::{self, Write};

/// Checks if the given string has "same digits" according to the problem's logic.
/// The input string is expected to contain only numeric characters.
fn has_same_digits(s: &mut Vec<u8>) -> bool {
    let mut len = s.len();
    let mut i = 0;

    // Convert characters to their numeric values (0-9).
    while i < len {
        s[i] &= 0x0f;
        i += 1;
    }

    // Perform the iterative reduction process.
    while len > 1 {
        len -= 1;
        for i in 0..len {
            s[i] = (s[i] + s[i + 1]) % 10;
        }
    }

    // Check if the first two digits are the same.
    s[0] == s[1]
}

fn main() {
    // Create a buffer for reading input.
    let mut input = String::new();

    // Read the input string from stdin.
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim the input to remove any trailing newline or whitespace.
    let input = input.trim();

    // Convert the input string into a vector of bytes for processing.
    let mut s: Vec<u8> = input.bytes().collect();

    // Call the function and print the result as 1 (true) or 0 (false).
    let result = has_same_digits(&mut s);
    println!("{}", if result { 1 } else { 0 });
}