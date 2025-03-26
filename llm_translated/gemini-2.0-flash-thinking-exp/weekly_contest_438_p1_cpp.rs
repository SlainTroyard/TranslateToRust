fn solve() {
    use std::io;

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    let result = Solution::has_same_digits(s.to_string()); // Pass owned String
    println!("{}", if result { 1 } else { 0 });
}

struct Solution {} // Define the Solution struct

impl Solution {
    pub fn has_same_digits(s_in: String) -> bool {
        let mut s = s_in; // Make s mutable

        while s.len() > 2 {
            let mut t = String::new();
            let s_bytes = s.as_bytes(); // Convert to bytes for efficient char access

            for i in 0..s.len() - 1 {
                // Convert chars to digits, perform addition modulo 10, and convert back to char
                let x = (s_bytes[i] - b'0') as u32;
                let y = (s_bytes[i + 1] - b'0') as u32;
                t.push(((x + y) % 10 + b'0') as char);
            }
            s = t; // Update s with the new string t
        }

        if s.len() == 2 {
            // Compare the first and second characters of the final string
            s.chars().nth(0) == s.chars().nth(1)
        } else if s.len() == 1 {
            true // If length is 1 after reduction, consider them same (vacuously true in original problem context, but handle for completeness)
        } else { // s.len() == 0, should not happen based on problem description and logic, but handle for completeness
            true // Consider empty string case as true as well.
        }
    }
}

fn main() {
    solve();
}