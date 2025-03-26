use std::io::{self, Write};

// Function to check if the number is balanced
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for c in num.chars() {
        total += '0' as i32 - c as i32;
        total = -total;
    }
    total == 0
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Trim any trailing whitespace or newline characters
    let num = input.trim();
    
    // Check if the number is balanced
    if is_balanced(num) {
        println!("true");
    } else {
        println!("false");
    }
}