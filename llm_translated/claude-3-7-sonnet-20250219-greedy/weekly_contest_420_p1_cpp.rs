// Problem: Weekly Contest 420 Problem 1
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn string_sequence(target: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut s = String::new();
        
        for c in target.chars() {
            // Add a placeholder character
            s.push('a');
            
            // For each character from 'a' to the target character
            for j in 'a'..=c {
                // Replace the last character with the current one
                let last_idx = s.len() - 1;
                s.replace_range(last_idx..=last_idx, &j.to_string());
                
                // Add the current string to the result
                ans.push(s.clone());
            }
        }
        
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the target string
    let target = lines.next().unwrap().unwrap();
    
    // Solve the problem
    let solution = Solution;
    let ans = Solution::string_sequence(target);
    
    // Print the result
    for s in ans {
        print!("{} ", s);
    }
    println!();
}