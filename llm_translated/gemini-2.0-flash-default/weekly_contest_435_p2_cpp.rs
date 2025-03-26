use std::cmp::{max, min};
use std::io;

struct Solution {}

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
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
            } else {
                x -= 1;
            }
            ans = max(ans, min(x.abs() + y.abs() + k * 2, (i + 1) as i32));
        }
        ans
    }
}

fn main() {
    // Read the input string and k from stdin
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim().to_string();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i32 = k_str.trim().parse().expect("Failed to parse k");

    // Create a Solution object and call the maxDistance method
    let sol = Solution {};
    let result = sol.max_distance(s, k);

    // Print the result to stdout
    println!("{}", result);
}