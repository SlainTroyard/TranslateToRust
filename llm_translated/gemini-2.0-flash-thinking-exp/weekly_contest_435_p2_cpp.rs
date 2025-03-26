use std::io;
use std::cmp::{max, min};

struct Solution {}

impl Solution {
    fn max_distance(&self, s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'N' {
                y += 1;
            } else if c == 'S' {
                y -= 1;
            } else if c == 'E' {
                x += 1;
            } else if c == 'W' {
                x -= 1;
            }
            ans = max(ans, min(x.abs() + y.abs() + k * 2, (i + 1) as i32));
        }
        ans
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");
    let parts: Vec<&str> = input_line.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().expect("Invalid input for k");

    let sol = Solution {};
    println!("{}", sol.max_distance(s, k));
}