use std::io;

struct Solution;

impl Solution {
    fn string_sequence(target: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut s = String::new();
        
        for c in target.chars() {
            // Add a placeholder 'a' to extend the string
            s.push('a');
            
            // For each character from 'a' to the target character,
            // create a new sequence and add it to the answer
            for j in ('a'..=c) {
                // Replace the last character with the current one
                s.replace_range((s.len() - 1)..s.len(), &j.to_string());
                ans.push(s.clone());
            }
        }
        
        ans
    }
}

fn main() {
    // Read the input string
    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read input");
    
    // Trim the input to remove the trailing newline
    let target = target.trim().to_string();
    
    // Call the solution function
    let solution = Solution;
    let ans = Solution::string_sequence(target);
    
    // Print the result with spaces between strings and a newline at the end
    for (i, s) in ans.iter().enumerate() {
        print!("{}", s);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    println!();
}