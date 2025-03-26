// Problem: Weekly Contest 422 Problem 1
use std::io;

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
    let mut input = String::new();
    
    // Input the number as a string
    // Assuming the number is no longer than 100 digits
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    // Trim any whitespace/newlines
    let num = input.trim();
    
    // Check if the number is balanced
    if is_balanced(num) {
        println!("true");
    } else {
        println!("false");
    }
}