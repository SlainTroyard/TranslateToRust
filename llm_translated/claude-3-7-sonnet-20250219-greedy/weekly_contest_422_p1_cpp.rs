// Problem: Weekly Contest 422 Problem 1
use std::io;

struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        for (i, c) in num.chars().enumerate() {
            let digit = c.to_digit(10).unwrap(); // Safe because we know input is a number
            if i % 2 == 0 {
                even_sum += digit;
            } else {
                odd_sum += digit;
            }
        }

        even_sum == odd_sum
    }
}

fn main() {
    let mut num = String::new();
    
    // Input the number as a string
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input");
    
    // Trim whitespace and newlines
    let num = num.trim().to_string();
    
    // Create a Solution object and call the is_balanced function
    let solution = Solution;
    let result = Solution::is_balanced(num);
    
    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}