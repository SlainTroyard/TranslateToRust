// Problem: Weekly Contest 420 Problem 1

use std::io;

fn main() {
    // Read the target string from stdin
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    target = target.trim().to_string(); // Trim any leading/trailing whitespace

    // Create an instance of the Solution struct and call the string_sequence method
    let solution = Solution;
    let ans = solution.string_sequence(target);

    // Print the result to stdout
    for s in ans {
        print!("{} ", s);
    }
    println!();
}

struct Solution;

impl Solution {
    /// Generates a sequence of strings based on the target string.
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();

        for c in target.chars() {
            s.push('a'); // Placeholder
            for j in 'a'..=c {
                s.pop(); // Remove the placeholder
                s.push(j); // Replace with the current character
                ans.push(s.clone()); // Add the current state of s to the answer
            }
        }

        ans
    }
}