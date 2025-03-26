// Problem: Weekly Contest 437 Problem 1

use std::io;

struct Solution;

impl Solution {
    // Check if the string `s` contains a substring of length `k` with all the same characters.
    pub fn has_special_substring(s: &str, k: usize) -> bool {
        let n = s.len();
        let mut cnt = 0;
        let bytes = s.as_bytes();

        for i in 0..n {
            cnt += 1;
            if i == n - 1 || bytes[i] != bytes[i + 1] {
                if cnt == k {
                    return true;
                }
                cnt = 0;
            }
        }

        false
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into the string and the integer
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0];
    let k: usize = parts[1].parse().expect("Failed to parse integer");

    // Create an instance of the Solution and call the function
    let sol = Solution;
    let result = sol.has_special_substring(s, k);

    // Print the result to stdout
    println!("{}", result);
}