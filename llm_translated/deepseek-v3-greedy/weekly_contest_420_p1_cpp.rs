use std::io::{self, Write};

struct Solution;

impl Solution {
    fn string_sequence(target: &str) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();
        
        for c in target.chars() {
            s.push('a'); // Placeholder
            for j in 'a'..=c {
                s.pop(); // Remove the last character
                s.push(j); // Replace with the current character
                ans.push(s.clone()); // Clone the string to avoid ownership issues
            }
        }
        
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target = input.trim(); // Remove any trailing whitespace or newline
    
    let solution = Solution;
    let ans = solution.string_sequence(target);
    
    for s in ans {
        print!("{} ", s);
    }
    println!(); // Ensure the output ends with a newline
}