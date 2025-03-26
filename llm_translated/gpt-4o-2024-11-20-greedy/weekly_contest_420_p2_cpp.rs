// Problem: Weekly Contest 420 Problem 2
use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: usize) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26]; // Array to count occurrences of each character

        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize; // Convert character to index
            cnt[idx] += 1;

            // Shrink the window from the left while the current character count is >= k
            while cnt[idx] >= k {
                let left_idx = (s.as_bytes()[left] - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }

            // Add the number of valid substrings ending at the current position
            ans += left as i32;
        }

        ans
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    // Split the input into the string `s` and the integer `k`
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("Missing string input").to_string();
    let k: usize = parts
        .next()
        .expect("Missing integer input")
        .parse()
        .expect("Failed to parse integer");

    // Create a solution instance and compute the result
    let sol = Solution;
    let result = sol.number_of_substrings(s, k);

    // Print the result to stdout
    println!("{}", result);
}