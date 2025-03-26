use std::cmp::{max, min};
use std::io::{self, Read};

fn max_difference(s: &str, k: usize) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();
    let s: Vec<usize> = s.chars().map(|c| c as usize - '0' as usize).collect();

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }

            let mut cur_s = [0; 5]; // Current window digit counts
            let mut pre_s = [0; 5]; // Left part digit counts
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum differences
            let mut left = 0;

            for i in 0..len {
                cur_s[s[i]] += 1; // Update current window digit counts
                let r = i + 1;

                // Shrink the left boundary of the window if conditions are met
                while r - left >= k
                    && cur_s[x] > pre_s[x]
                    && cur_s[y] > pre_s[y]
                {
                    let p = &mut min_s[pre_s[x] & 1][pre_s[y] & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]); // Update minimum difference
                    pre_s[s[left]] += 1; // Update left part digit counts
                    left += 1;
                }

                // Update the answer
                ans = max(
                    ans,
                    cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1) ^ 1][cur_s[y] & 1],
                );
            }
        }
    }

    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();

    // Parse the string `s` and integer `k`
    let s = lines.next().expect("Missing string input");
    let k: usize = lines
        .next()
        .expect("Missing integer input")
        .parse()
        .expect("Failed to parse integer");

    // Call the function and compute the result
    let result = max_difference(s, k);

    // Print the result
    println!("{}", result);
}