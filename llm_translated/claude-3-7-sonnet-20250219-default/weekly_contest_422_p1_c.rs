// Problem: Weekly Contest 422 Problem 1
use std::io;

// Function to check if the number is balanced
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for c in num.chars() {
        // In the C code, '0' - *num is calculating the negative of the digit value
        // When we do '0' as u8 - c as u8, we'd get an underflow in Rust
        // So we implement the logic more directly
        let digit_value = c as i32 - '0' as i32;
        total += -digit_value; // Add negative of digit
        total = -total;        // Negate the total
    }
    total == 0
}

fn main() {
    // Buffer for storing the input
    let mut num = String::new();
    
    // Read the input from stdin
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    
    // Trim to remove any whitespace/newlines
    let num = num.trim();
    
    // Check if the number is balanced
    if is_balanced(num) {
        println!("true");
    } else {
        println!("false");
    }
}