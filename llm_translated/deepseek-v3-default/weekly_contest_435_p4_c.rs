use std::cmp::{max, min};
use std::io::{self, Write};

fn max_difference(s: &str, k: i32) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();
    let s_bytes = s.as_bytes();

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }

            let mut cur_s = [0; 5]; // Current window counts
            let mut pre_s = [0; 5]; // Left window counts
            let mut min_s = [[INF, INF], [INF, INF]]; // Minimum differences
            let mut left = 0;

            for i in 0..len {
                cur_s[(s_bytes[i] - b'0') as usize] += 1;
                let r = i + 1;

                // Shrink the window from the left if necessary
                while (r - left) as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[pre_s[x] as usize & 1][pre_s[y] as usize & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]);
                    pre_s[(s_bytes[left] - b'0') as usize] += 1;
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }
    let s = parts[0];
    let k: i32 = parts[1].parse().expect("Invalid integer for k");

    let result = max_difference(s, k);
    println!("{}", result);
}