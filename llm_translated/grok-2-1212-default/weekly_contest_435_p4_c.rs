// Problem: Weekly Contest 435 Problem 4
use std::io::{self, Read};

// Returns the larger of two numbers
fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

// Returns the smaller of two numbers
fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

// Main function implementation
fn max_difference(s: &str, k: i32) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }

            let mut cur_s = [0; 5]; // Current window count of each digit
            let mut pre_s = [0; 5]; // Count of digits in the left part of the window
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum difference
            let mut left = 0;

            for (i, &c) in s.as_bytes().iter().enumerate() {
                cur_s[(c - b'0') as usize] += 1; // Update current window count
                let r = i + 1;

                // Try to shrink the left boundary of the window when conditions are met
                while r as i32 - left as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[(pre_s[x] & 1) as usize][(pre_s[y] & 1) as usize];
                    *p = min(*p, pre_s[x] - pre_s[y]); // Update minimum difference
                    pre_s[(s.as_bytes()[left] - b'0') as usize] += 1; // Update left part count
                    left += 1;
                }

                // Update answer
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[((cur_s[x] & 1) ^ 1) as usize][(cur_s[y] & 1) as usize]);
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.trim().split_whitespace();

    let s = lines.next().unwrap_or("");
    let k: i32 = lines.next().unwrap_or("0").parse().unwrap_or(0);

    // Calculate result
    let result = max_difference(s, k);

    // Output result
    println!("{}", result);

    Ok(())
}