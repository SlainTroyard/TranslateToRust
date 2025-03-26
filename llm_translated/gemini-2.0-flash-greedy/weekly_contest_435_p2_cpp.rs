use std::cmp::{max, min, abs};
use std::io;
use std::io::Read;

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
            ans = max(ans, min(abs(x) + abs(y) + k * 2, (i + 1) as i32));
        }

        ans
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Split input into lines and parse values
    let mut lines = input.lines();
    let s = lines.next().unwrap().to_string();
    let k: i32 = lines.next().unwrap().parse()?;

    // Create an instance of Solution and call the method
    let sol = Solution {};
    let result = sol.max_distance(s, k);

    // Print the result to stdout
    println!("{}", result);

    Ok(())
}