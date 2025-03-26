// Problem: Weekly Contest 422 Problem 1

use std::io;

/// Function to check if the number is balanced
fn is_balanced(num: &str) -> bool {
    let mut total = 0;
    for c in num.chars() {
        total += '0' as i32 - c as i32;
        total = -total;
    }
    total == 0
}

fn main() {
    // Assuming the number is no longer than 100 digits
    let mut num = String::with_capacity(101);

    // Input the number as a string
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    // Remove the newline character if present
    num = num.trim().to_string();

    // Check if the number is balanced
    if is_balanced(&num) {
        println!("true");
    } else {
        println!("false");
    }
}