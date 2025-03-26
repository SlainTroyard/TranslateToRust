// Problem: Weekly Contest 435 Problem 2
use std::cmp::{max, min};
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distance(s: &str, k: usize) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;

        for (i, ch) in s.chars().enumerate() {
            match ch {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => panic!("Unexpected character in input!"),
            }
            // The formula max(ans, min(abs(x) + abs(y) + k * 2, i + 1)) is preserved
            ans = max(ans, min((x.abs() + y.abs() + (k * 2) as i32), (i + 1) as i32));
        }

        ans
    }
}

fn main() {
    // Read input via stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first input line as the string `s`
    let s = lines
        .next()
        .expect("Expected input string.")
        .expect("Failed to read line.");

    // Parse the second input line as the integer `k`
    let k = lines
        .next()
        .expect("Expected input integer.")
        .expect("Failed to read line.")
        .trim()
        .parse::<usize>()
        .expect("Failed to parse integer.");

    // Create an instance of the `Solution` struct and call the `max_distance` method
    let sol = Solution;
    let result = sol.max_distance(&s, k);

    // Print the result to stdout
    println!("{}", result);
}