use std::io;

struct Solution;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
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
                for (i, c) in s.bytes().enumerate() {
                    let digit = (c - b'0') as usize;
                    cur_s[digit] += 1;
                    let r = i + 1;
                    while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let px = (pre_s[x] & 1) as usize;
                        let py = (pre_s[y] & 1) as usize;
                        min_s[px][py] = min_s[px][py].min(pre_s[x] - pre_s[y]);
                        let left_c = s.as_bytes()[left];
                        let left_digit = (left_c - b'0') as usize;
                        pre_s[left_digit] += 1;
                        left += 1;
                    }
                    let x_parity = (cur_s[x] & 1) ^ 1;
                    let y_parity = cur_s[y] & 1;
                    let min_val = min_s[x_parity as usize][y_parity as usize];
                    let current_diff = (cur_s[x] - cur_s[y]) - min_val;
                    if current_diff > ans {
                        ans = current_diff;
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 2 {
        panic!("Invalid input");
    }
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().unwrap();
    let ans = Solution::max_difference(s, k);
    println!("{}", ans);
}