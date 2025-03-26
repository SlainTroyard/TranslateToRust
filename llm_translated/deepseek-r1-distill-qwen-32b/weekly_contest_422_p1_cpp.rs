use std::io;

struct Solution;

impl Solution {
    /// Checks if the sum of digits at even positions equals the sum at odd positions.
    /// The positions are 0-based, meaning the first digit is at position 0 (even).
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
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    num = num.trim().to_string();

    let result = Solution::is_balanced(&num);
    if result {
        println!("true");
    } else {
        println!("false");
    }
}