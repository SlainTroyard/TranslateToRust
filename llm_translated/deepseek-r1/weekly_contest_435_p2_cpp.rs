use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut ans = 0;
        
        // Iterate through each character in the string with its index
        for (i, c) in s.chars().enumerate() {
            // Update coordinates based on the current character
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => {} // Invalid characters are ignored as per original logic
            }
            
            // Calculate current maximum possible distance
            let current_value = (x.abs() + y.abs() + 2 * k).min((i + 1) as i32);
            ans = ans.max(current_value);
        }
        
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();
    
    // Read input line and split into components
    let input_line = input_lines.next().unwrap().expect("Failed to read input");
    let parts: Vec<&str> = input_line.split_whitespace().collect();
    
    // Parse input values
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().expect("Invalid integer for k");
    
    // Compute and print result
    println!("{}", Solution::max_distance(s, k));
}