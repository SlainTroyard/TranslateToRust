use std::io::{self, Write};

// Function to check if the number is balanced
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for ch in num.chars() {
        total += '0' as i32 - ch as i32;
        total = -total;
    }
    total == 0
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim any trailing whitespace or newlines
    let num = input.trim();
    
    // Check if the number is balanced
    if is_balanced(num) {
        println!("true");
    } else {
        println!("false");
    }
}