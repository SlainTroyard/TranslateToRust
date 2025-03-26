// Problem: Weekly Contest 435 Problem 2
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_distance(s: &str, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => {}
            }
            ans = ans.max((x.abs() + y.abs() + k * 2).min(i as i32 + 1));
        }
        ans
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input
    let s = lines.next().unwrap().expect("Failed to read string");
    let k = lines.next().unwrap().expect("Failed to read integer")
        .parse::<i32>().expect("Failed to parse integer");

    // Create an instance of the Solution and call the function
    let sol = Solution;
    let result = sol.max_distance(&s, k);

    // Write the result to stdout
    println!("{}", result);
}