// Problem: Weekly Contest 420 Problem 1
use std::io::{self, BufRead};

fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;
    
    // Calculate the total number of strings in the sequence
    for c in target.chars() {
        return_size += (c as u8 - b'a' + 1) as usize;
    }
    
    let mut ans: Vec<String> = Vec::with_capacity(return_size);
    let mut l = 0;
    
    // Generate the sequence according to the problem logic
    for t in target.chars() {
        for c in 'a'..=t {
            let mut new_str = String::with_capacity(l + 1);
            
            // Copy first l characters from target
            new_str.push_str(&target[0..l]);
            
            // Add the current character
            new_str.push(c);
            
            ans.push(new_str);
        }
        l += 1;
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the target string
    if let Some(Ok(target)) = lines.next() {
        let result = string_sequence(&target);
        
        // Print the results with spaces between them
        for s in &result {
            print!("{} ", s);
        }
        println!();
    }
}