// Problem: Weekly Contest 430 Problem 2
use std::cmp;
use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn answer_string(s: &str, k: usize) -> String {
        if k == 1 {
            return s.to_string();
        }
        let n = s.len();
        let mut ans = String::new();

        // Iterate through all substrings while obeying the logic of the problem
        for i in 0..n {
            let end = n - cmp::max(k - 1, i);
            if let Some(substring) = s.get(i..end) {
                ans = cmp::max(ans, substring.to_string()); // Compare maximum lexicographically
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    let mut stdout = io::stdout();

    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().to_string();
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k = input.trim().parse::<usize>().expect("Failed to parse k");

    // Call answer_string and print the result
    let solution = Solution;
    let result = solution.answer_string(&s, k);
    
    // Output result to stdout
    writeln!(stdout, "{}", result).expect("Failed to write to stdout");
}