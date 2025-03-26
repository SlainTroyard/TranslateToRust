// Problem: Weekly Contest 422 Problem 1
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
    let mut input = String::new();
    
    // Read the input number as a string
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Trim any trailing whitespace, such as newline or spaces
    let num = input.trim();
    
    // Check if the number is balanced
    if is_balanced(num) {
        println!("true");
    } else {
        println!("false");
    }
}