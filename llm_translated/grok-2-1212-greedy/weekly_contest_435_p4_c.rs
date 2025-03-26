use std::io::{self, BufRead};

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

            let mut cur_s = [0; 5]; // Count of each digit in the current window
            let mut pre_s = [0; 5]; // Count of each digit in the left part of the window
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum difference
            let mut left = 0;

            for (i, &c) in s.as_bytes().iter().enumerate() {
                cur_s[(c - b'0') as usize] += 1; // Update the count of digits in the current window
                let r = i + 1;

                // Try to shrink the left boundary of the window when the window size >= k and other conditions are met
                while r as i32 - left as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[pre_s[x] as usize & 1][pre_s[y] as usize & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]); // Update the minimum difference
                    pre_s[(s.as_bytes()[left] - b'0') as usize] += 1; // Update the count of digits in the left part of the window
                    left += 1;
                }

                // Update the answer
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1) ^ 1][cur_s[y] & 1 as usize]);
            }
        }
    }

    ans
}

fn main() -> io::Result<()> {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(line) = lines.next() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Error reading input");
            std::process::exit(1);
        }

        let s = parts[0];
        let k: i32 = parts[1].parse().map_err(|_| {
            eprintln!("Error parsing k");
            std::process::exit(1);
        })?;

        // Call the function to calculate the result
        let result = max_difference(s, k);

        // Output the result
        println!("{}", result);
    }

    Ok(())
}