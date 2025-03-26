use std::io;
use std::cmp::{max, min};

struct Solution {}

impl Solution {
    fn max_distance(&self, s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        
        for (i, ch) in s.chars().enumerate() {
            match ch {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => (), // Ignore any other characters
            }
            
            // Calculate the minimum of Manhattan distance + 2k and current position
            // Convert i+1 to i32 to match the type of other operands
            ans = max(ans, min(x.abs() + y.abs() + k * 2, (i + 1) as i32));
        }
        
        ans
    }
}

fn main() {
    // Read the string s
    let mut s = String::new();
    io::stdin().read_line(&mut s)
        .expect("Failed to read the input string");
    let s = s.trim().to_string();
    
    // Read the integer k
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input)
        .expect("Failed to read the k value");
    let k: i32 = k_input.trim().parse()
        .expect("Invalid integer input for k");
    
    let sol = Solution {};
    println!("{}", sol.max_distance(s, k));
}