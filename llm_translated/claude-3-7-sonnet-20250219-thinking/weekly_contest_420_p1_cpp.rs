use std::io;

struct Solution;

impl Solution {
    fn string_sequence(target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();
        
        // Process each character in the target string
        for c in target.chars() {
            // Add placeholder character 'a'
            s.push('a');
            
            // Replace the placeholder with each character from 'a' to current character
            for j in 'a'..=c {
                // Replace the last character with the current one
                let last_index = s.len() - 1;
                s.replace_range(last_index..=last_index, &j.to_string());
                ans.push(s.clone());
            }
        }
        
        ans
    }
}

fn main() {
    // Read input from stdin
    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("Failed to read input");
    let target = target.trim().to_string();
    
    // Call the solution function
    let ans = Solution::string_sequence(target);
    
    // Print the output in the same format as the original code
    for s in ans {
        print!("{} ", s);
    }
    println!();
}