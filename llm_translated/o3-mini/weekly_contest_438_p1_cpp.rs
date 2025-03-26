use std::io::{self, BufRead};

// Define the Solution struct to mirror the C++ class structure
struct Solution;

impl Solution {
    /// Given a string of digits, repeatedly sum adjacent digits (modulo 10)
    /// until only two digits remain, then check if they are the same.
    fn has_same_digits(&self, mut s: String) -> bool {
        // Loop until the string has exactly 2 characters
        while s.len() > 2 {
            // It's more convenient to work with the bytes of the string
            let bytes = s.as_bytes();
            let mut t = String::with_capacity(s.len() - 1);
            // Iterate over adjacent pairs in the string
            for i in 0..bytes.len() - 1 {
                // Convert each char ('0' to '9') into its numeric value
                let x = (bytes[i] - b'0') as u32;
                let y = (bytes[i + 1] - b'0') as u32;
                // Compute the sum modulo 10, then convert it back to a char
                let digit = ((x + y) % 10) as u8;
                t.push((digit + b'0') as char);
            }
            // Replace s with the newly computed string
            s = t;
        }
        // Check if the two digits are the same
        // We trust s always has length 2 at this point.
        s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap()
    }
}

fn main() -> io::Result<()> {
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // According to the C++ code, we read a single token from input.
    // It may be from a line containing multiple tokens, thus we split whitespaces.
    let mut s = String::new();
    'outer: while let Some(line) = lines.next() {
        let line = line?;  // Handle potential IO errors.
        for token in line.split_whitespace() {
            s = token.to_string();
            break 'outer;
        }
    }

    // If no valid input was provided, exit the program.
    if s.is_empty() {
        return Ok(());
    }

    let sol = Solution;
    let result = sol.has_same_digits(s);

    // Output the boolean as in C++ (printing with cout yields 1 for true, 0 for false).
    println!("{}", if result { 1 } else { 0 });
    Ok(())
}