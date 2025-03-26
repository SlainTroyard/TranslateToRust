// Problem: Weekly Contest 422 Problem 1
use std::io;

struct Solution;

impl Solution {
    fn is_balanced(num: &str) -> bool {
        let mut even_sum = 0;
        let mut odd_sum = 0;

        for (i, c) in num.chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
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
    // Input the number as a string
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input");
    
    // Trim the input to remove any whitespace or newline characters
    let num = num.trim();
    
    // Create a Solution object and call the is_balanced function
    let solution = Solution;
    let result = solution.is_balanced(num);
    
    // Output the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}