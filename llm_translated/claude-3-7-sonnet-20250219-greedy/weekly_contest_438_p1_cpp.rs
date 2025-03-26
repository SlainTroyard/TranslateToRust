use std::io;

struct Solution;

impl Solution {
    pub fn has_same_digits(mut s: String) -> bool {
        // Continue processing until the string has 2 or fewer characters
        while s.len() > 2 {
            let mut t = String::new();
            
            // For each adjacent pair of digits, compute their sum modulo 10
            for i in 0..s.len() - 1 {
                let x = s.as_bytes()[i] - b'0';
                let y = s.as_bytes()[i + 1] - b'0';
                t.push(((x + y) % 10 + b'0') as char);
            }
            
            // Replace s with the new string for the next iteration
            s = t;
        }
        
        // Check if the two remaining digits are the same
        s.as_bytes()[0] == s.as_bytes()[1]
    }
}

fn main() {
    // Read input string
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.trim().to_string(); // Remove trailing newline
    
    // Create solution instance and call the method
    let sol = Solution;
    
    // Print the result (1 for true, 0 for false, matching C++ output)
    println!("{}", if sol.has_same_digits(s) { 1 } else { 0 });
}