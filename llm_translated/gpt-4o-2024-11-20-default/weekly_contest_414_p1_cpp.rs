// Problem: Weekly Contest 414 Problem 1

use std::io;
use std::str::FromStr;

struct Solution;

impl Solution {
    fn convert_date_to_binary(date: &str) -> String {
        // Helper function to convert integer to binary string without leading zeros
        let bin = |x: u32| -> String {
            format!("{:b}", x) // Convert number to binary and omit leading zeros
        };

        if let (Some(year), Some(month), Some(day)) = (
            date.get(0..4).and_then(|s| u32::from_str(s).ok()),
            date.get(5..7).and_then(|s| u32::from_str(s).ok()),
            date.get(8..10).and_then(|s| u32::from_str(s).ok()),
        ) {
            return format!("{}-{}-{}", bin(year), bin(month), bin(day));
        } else {
            panic!("Invalid date format");
        }
    }
}

fn main() {
    // Handle standard input and output
    let mut date = String::new();
    
    // Read the date string from stdin
    io::stdin().read_line(&mut date).expect("Failed to read input");
    let date = date.trim(); // Trim newline to match input format
    
    // Create an instance of the solution
    let solution = Solution;
    
    // Output the result to stdout
    println!("{}", solution.convert_date_to_binary(date));
}