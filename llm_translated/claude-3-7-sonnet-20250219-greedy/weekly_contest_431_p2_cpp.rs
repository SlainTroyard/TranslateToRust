use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn calculate_score(s: String) -> i64 {
        // Create 26 stacks for each letter of the alphabet
        let mut stacks: [Vec<usize>; 26] = Default::default();
        let mut ans: i64 = 0;
        
        for (i, ch) in s.chars().enumerate() {
            let c = (ch as u8 - b'a') as usize;
            
            // Check if there's a complementary character (z-a, y-b, etc.)
            if !stacks[25 - c].is_empty() {
                // Calculate score as current position minus position of complementary character
                let pos = stacks[25 - c].pop().unwrap();
                ans += (i - pos) as i64;
            } else {
                // Push current character's position to its stack
                stacks[c].push(i);
            }
        }
        
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the string
    let s = lines.next().unwrap().unwrap();
    
    let sol = Solution;
    println!("{}", Solution::calculate_score(s));
}