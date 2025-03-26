// Problem: Weekly Contest 437 Problem 1

use std::io;

struct Solution;

impl Solution {
    // Method to check for a special substring
    pub fn has_special_substring(s: &str, k: usize) -> bool {
        let n = s.len();
        let mut cnt = 0; // Consecutive character count
        let chars: Vec<char> = s.chars().collect(); // Convert string into a vector of characters

        for i in 0..n {
            cnt += 1;
            // Check if we are at the last character, or the current character is different from the next
            if i == n - 1 || chars[i] != chars[i + 1] {
                if cnt == k {
                    return true;
                }
                cnt = 0; // Reset the counter upon encountering a different character
            }
        }
        false
    }
}

fn main() {
    // Handle input/output from stdin/stdout
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut inputs = input.split_whitespace();

    let s = inputs.next().unwrap_or("").to_string(); // First line: the string `s`
    let k: usize = inputs.next().unwrap_or("0").parse().expect("Failed to parse integer");

    let sol = Solution;
    let result = sol.has_special_substring(&s, k);
    println!("{}", result); // 1 for true, 0 for false
}