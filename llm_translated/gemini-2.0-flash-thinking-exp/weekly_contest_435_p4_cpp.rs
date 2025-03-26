use std::io;
use std::cmp::{max, min};

struct Solution {}

impl Solution {
    fn max_difference(&self, s: String, k: i32) -> i32 {
        let inf = i32::MAX / 2;
        let mut ans = -inf;
        for x in 0..5 {
            for y in 0..5 {
                if y == x {
                    continue;
                }
                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[inf; 2]; 2];
                let mut left = 0;
                let s_bytes = s.as_bytes(); // Convert string to bytes for faster access
                for i in 0..s.len() {
                    let digit = (s_bytes[i] - b'0') as usize; // Efficiently convert byte to digit
                    cur_s[digit] += 1;
                    let r = i + 1;
                    while r as i32 - left as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let p = &mut min_s[(pre_s[x] & 1) as usize][(pre_s[y] & 1) as usize];
                        *p = min(*p, pre_s[x] - pre_s[y]);
                        let left_digit = (s_bytes[left] - b'0') as usize;
                        pre_s[left_digit] += 1;
                        left += 1;
                    }
                    ans = max(ans, cur_s[x] - cur_s[y] - min_s[((cur_s[x] & 1) ^ 1) as usize][(cur_s[y] & 1) as usize]);
                }
            }
        }
        ans
    }
}

fn main() {
    let mut s = String::new();
    let mut k_str = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut k_str).expect("Failed to read line");

    let s = s.trim().to_string();
    let k: i32 = k_str.trim().parse().expect("Invalid input for k");

    let sol = Solution {};
    println!("{}", sol.max_difference(s, k));
}