use std::io::{self, Write};

// Function to check if the processed digits are the same
fn has_same_digits(s: &mut String) -> bool {
    let len = s.len();
    let mut chars: Vec<u8> = s.bytes().collect();

    // Convert each character to its corresponding digit by masking with 0x0F
    for i in 0..len {
        chars[i] &= 0x0F;
    }

    // Perform the digit reduction process
    let mut current_len = len;
    while current_len > 1 {
        for i in 0..current_len - 1 {
            chars[i] = (chars[i] + chars[i + 1]) % 10;
        }
        current_len -= 1;
    }

    // Check if the remaining two digits are the same
    chars[0] == chars[1]
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Trim any trailing newline characters
    let input = input.trim();

    // Check if the processed digits are the same
    let result = has_same_digits(&mut input.to_string());

    // Print the result as a boolean (0 or 1)
    println!("{}", result as i32);
}