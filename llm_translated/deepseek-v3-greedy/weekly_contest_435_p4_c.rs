use std::cmp::{max, min};
use std::io::{self, Write};

// Returns the maximum difference based on the given algorithm
fn max_difference(s: &str, k: usize) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();
    let s_bytes = s.as_bytes();

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }

            let mut cur_s = [0; 5]; // Current window counts for each digit
            let mut pre_s = [0; 5]; // Prefix window counts for each digit
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum differences
            let mut left = 0;

            for i in 0..len {
                cur_s[(s_bytes[i] - b'0') as usize] += 1; // Update current window counts
                let r = i + 1;

                // Shrink the window from the left if conditions are met
                while r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[pre_s[x] as usize & 1][pre_s[y] as usize & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]); // Update minimum difference
                    pre_s[(s_bytes[left] - b'0') as usize] += 1; // Update prefix window counts
                    left += 1;
                }

                // Update the answer
                ans = max(
                    ans,
                    cur_s[x] - cur_s[y] - min_s[(cur_s[x] as usize & 1) ^ 1][cur_s[y] as usize & 1],
                );
            }
        }
    }

    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("No string input");
    let k: usize = parts
        .next()
        .expect("No k input")
        .parse()
        .expect("k is not a number");

    // Compute the result
    let result = max_difference(s, k);

    // Output the result
    println!("{}", result);
}