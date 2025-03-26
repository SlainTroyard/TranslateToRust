use std::io;

struct Solution {}

impl Solution {
    pub fn has_same_digits(mut s: String) -> bool {
        // Keep transforming the string until it's only 2 digits
        while s.len() > 2 {
            let mut t = String::new();
            // For each pair of consecutive digits, add them and take modulo 10
            for i in 0..s.len() - 1 {
                let x = s.chars().nth(i).unwrap() as u8 - b'0';
                let y = s.chars().nth(i + 1).unwrap() as u8 - b'0';
                let sum_digit = ((x + y) % 10) + b'0';
                t.push(sum_digit as char);
            }
            s = t;
        }
        // Check if the final 2 digits are the same
        s.chars().nth(0) == s.chars().nth(1)
    }
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read input");
    
    // Trim whitespace and newlines
    let s = s.trim().to_string();
    
    // Create solution instance and call the method
    let sol = Solution {};
    
    // Output the result (1 for true, 0 for false)
    println!("{}", sol.has_same_digits(s) as i32);
}