use std::cmp::{max, min};
use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let inf = i32::MAX / 2;
        let mut ans = -inf;
        let s_bytes = s.as_bytes();

        for x in 0..5 {
            for y in 0..5 {
                if y == x {
                    continue;
                }

                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[inf, inf], [inf, inf]];
                let mut left = 0;

                for i in 0..s_bytes.len() {
                    let digit = (s_bytes[i] - b'0') as usize;
                    cur_s[digit] += 1;
                    let r = i + 1;

                    while r as i32 - left as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let p = &mut min_s[(pre_s[x] & 1) as usize][(pre_s[y] & 1) as usize];
                        *p = min(*p, pre_s[x] - pre_s[y]);
                        let left_digit = (s_bytes[left] - b'0') as usize;
                        pre_s[left_digit] += 1;
                        left += 1;
                    }

                    ans = max(
                        ans,
                        cur_s[x] - cur_s[y] - min_s[((cur_s[x] & 1) ^ 1) as usize][(cur_s[y] & 1) as usize],
                    );
                }
            }
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut iter = s.trim().split_whitespace();
    let s = iter.next().unwrap().to_string();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let sol = Solution {};
    println!("{}", sol.max_difference(s, k));
    Ok(())
}