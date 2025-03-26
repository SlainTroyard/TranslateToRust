use std::io;

struct Solution;

impl Solution {
    fn has_same_digits(mut s: String) -> bool {
        while s.len() > 2 {
            let mut t = String::with_capacity(s.len() - 1);
            let s_bytes = s.as_bytes();
            for i in 0..(s.len() - 1) {
                // Get the numeric value of each digit by subtracting ASCII value of '0'
                let x = s_bytes[i] - b'0';
                let y = s_bytes[i + 1] - b'0';
                // Add digits, take modulo 10, and convert back to char
                t.push(char::from((x + y) % 10 + b'0'));
            }
            s = t;
        }
        
        // Check if the last two remaining digits are the same
        let s_bytes = s.as_bytes();
        s_bytes[0] == s_bytes[1]
    }
}

fn main() {
    // Read input from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    // Trim whitespace and newlines
    let s = s.trim().to_string();
    
    let sol = Solution;
    // Output 1 for true and 0 for false to match C++ output format
    println!("{}", sol.has_same_digits(s) as i32);
}