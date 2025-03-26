// Problem: Weekly Contest 420 Problem 1
use std::io::{self, BufRead};

fn string_sequence(target: &str) -> Vec<String> {
    let mut return_size = 0;
    
    // Calculate the total number of strings we'll generate
    for c in target.chars() {
        return_size += (c as u8 - b'a' + 1) as usize;
    }
    
    let mut ans = Vec::with_capacity(return_size);
    let mut i = 0;
    
    // For each position in the target string
    for (l, t_char) in target.chars().enumerate() {
        // For each character from 'a' to the current character in target
        for c in 'a'..=t_char {
            // Create a new string with the prefix from target and append the current character
            let mut new_str = String::with_capacity(l + 1);
            new_str.push_str(&target[..l]);
            new_str.push(c);
            
            ans.push(new_str);
            i += 1;
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the target string
    if let Some(Ok(target)) = lines.next() {
        let ans = string_sequence(&target);
        
        // Print the results
        for s in ans {
            print!("{} ", s);
        }
        println!();
    }
}