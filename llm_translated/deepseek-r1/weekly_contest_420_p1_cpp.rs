use std::io;

struct Solution;

impl Solution {
    fn string_sequence(target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = Vec::new();  // Using bytes for efficient mutation
        
        for c in target.chars() {
            let current_c = c as u8;
            s.push(b'a');  // Add placeholder
            let last_idx = s.len() - 1;
            
            // Iterate from 'a' to current character (inclusive)
            for j in b'a'..=current_c {
                s[last_idx] = j;  // Update last character
                ans.push(String::from_utf8(s.clone()).unwrap());  // Clone and convert to String
            }
        }
        
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target = input.split_whitespace().next().expect("No input provided").to_string();
    
    let ans = Solution::string_sequence(target);
    
    // Print results with exact same format as original code
    for s in ans {
        print!("{} ", s);
    }
    println!();  // Final newline
}