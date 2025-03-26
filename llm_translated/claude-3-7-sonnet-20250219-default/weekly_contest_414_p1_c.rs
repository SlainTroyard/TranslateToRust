// Problem: Weekly Contest 414 Problem 1
use std::io::{self, Read};

fn convert_date_to_binary(date: &str) -> String {
    // Initialize parts for year, month, and day
    let y = date[0..4].parse::<i32>().unwrap_or(0);
    let m = date[5..7].parse::<i32>().unwrap_or(0);
    let d = date[8..10].parse::<i32>().unwrap_or(0);
    
    // Convert year to binary
    let y_bin = if y == 0 {
        "0".to_string()
    } else {
        format!("{:b}", y)
    };
    
    // Convert month to binary
    let m_bin = if m == 0 {
        "0".to_string()
    } else {
        format!("{:b}", m)
    };
    
    // Convert day to binary
    let d_bin = if d == 0 {
        "0".to_string()
    } else {
        format!("{:b}", d)
    };
    
    // Combine all parts with hyphens
    format!("{}-{}-{}", y_bin, m_bin, d_bin)
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim any whitespace and newlines
    let date = input.trim();
    
    // Convert the date and print the result
    let result = convert_date_to_binary(date);
    println!("{}", result);
}