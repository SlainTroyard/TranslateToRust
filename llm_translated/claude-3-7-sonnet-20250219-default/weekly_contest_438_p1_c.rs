use std::io::{self, BufRead};

/// Checks if a string has same digits after repeatedly computing pairwise sums modulo 10
/// 
/// This function works by:
/// 1. Converting each character to its numeric value
/// 2. Repeatedly computing pairwise sums modulo 10 until only two digits remain
/// 3. Checking if the final two digits are equal
fn has_same_digits(s: &str) -> bool {
    // Create a mutable vector of digits from the input string
    let mut digits: Vec<u8> = s.chars()
        .map(|c| (c as u8) & 0x0f) // Equivalent to c & 0x0f in C to get numeric value
        .collect();
    
    let mut len = digits.len();
    
    // Repeatedly compute pairwise sums until only two digits remain
    while len > 2 {
        for i in 0..len-1 {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
        len -= 1;
    }
    
    // Check if the final two digits are equal
    digits[0] == digits[1]
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string input
    if let Some(Ok(s)) = lines.next() {
        // Call the function and print the result
        let result = has_same_digits(&s);
        println!("{}", if result { 1 } else { 0 });
    }
}