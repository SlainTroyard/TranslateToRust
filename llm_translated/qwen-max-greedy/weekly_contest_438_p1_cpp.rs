// Problem: Weekly Contest 438 Problem 1

use std::io;

struct Solution;

impl Solution {
    // Check if the string has the same digits after the transformation
    fn has_same_digits(s: &str) -> bool {
        let mut s = s.to_string();
        while s.len() > 2 {
            let mut t = String::new();
            for i in 0..s.len() - 1 {
                let x = s.chars().nth(i).unwrap().to_digit(10).unwrap();
                let y = s.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
                t.push_str(&((x + y) % 10).to_string());
            }
            s = t;
        }
        s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap()
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Trim any leading or trailing whitespace

    // Create an instance of the Solution and call the function
    let sol = Solution;
    let result = sol.has_same_digits(input);

    // Write the result to stdout
    println!("{}", if result { "true" } else { "false" });
}