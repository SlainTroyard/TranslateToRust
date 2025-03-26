use std::cmp::{max, min};
use std::io::{self, BufRead};

const INF: i32 = i32::MAX / 2;

// Function to compute the maximum difference
fn max_difference(s: &str, k: usize) -> i32 {
    let len = s.len();
    let mut ans = -INF;

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }

            let mut cur_s = vec![0; 5]; // Current window's count per digit
            let mut pre_s = vec![0; 5]; // Left window part's count per digit
            let mut min_s = vec![vec![INF; 2]; 2]; // Minimum differences
            let mut left = 0;

            for (i, ch) in s.chars().enumerate() {
                let digit = (ch as u8 - b'0') as usize;
                cur_s[digit] += 1; // Update the current window digit count
                let r = i + 1;

                // Adjust left boundary of the window to satisfy conditions
                while r - left >= k
                    && cur_s[x] > pre_s[x]
                    && cur_s[y] > pre_s[y]
                {
                    let p = &mut min_s[pre_s[x] & 1][pre_s[y] & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]); // Update minimum difference
                    pre_s[(s.chars().nth(left).unwrap() as u8 - b'0') as usize] += 1; // Update left part counts
                    left += 1;
                }

                // Update answer
                ans = max(
                    ans,
                    cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1) ^ 1][cur_s[y] & 1],
                );
            }
        }
    }

    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the input
    let s = lines.next().unwrap()?.trim().to_string();
    let k: usize = lines.next().unwrap()?.trim().parse()?;

    // Compute the result and output it
    let result = max_difference(&s, k);
    println!("{}", result);

    Ok(())
}