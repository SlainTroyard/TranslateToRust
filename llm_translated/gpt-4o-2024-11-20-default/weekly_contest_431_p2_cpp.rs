// Problem: Weekly Contest 431 Problem 2
use std::io::{self, Write};
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn calculate_score(s: &str) -> i64 {
        // Stack arrays for both sides (0..25 for original characters and 25..0 for complements)
        let mut stacks: [VecDeque<usize>; 26] = Default::default();
        let mut ans = 0_i64;

        for (i, ch) in s.chars().enumerate() {
            // Translate character to an index between 0 and 25
            let current = (ch as u8 - b'a') as usize;
            // Complementary character index (25 - current)
            let complement = 25 - current;

            if let Some(&top_index) = stacks[complement].front() {
                // Complementary character exists in the stack
                ans += i as i64 - top_index as i64;
                stacks[complement].pop_front();
            } else {
                // Push character index to its respective stack
                stacks[current].push_front(i);
            }
        }

        ans
    }
}

fn main() {
    // Read input string from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Remove newline characters

    // Instantiate the solution and calculate the result
    let result = Solution::calculate_score(s);

    // Output the result to stdout
    let mut stdout = io::stdout();
    writeln!(stdout, "{}", result).expect("Failed to write output");
}