use std::io::{self, BufRead};
use std::cmp::{max, min};

const INF: i32 = i32::MAX / 2;

fn max_difference(s: &str, k: usize) -> i32 {
    let mut ans = -INF;
    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }
            let mut cur_s = [0; 5];
            let mut pre_s = [0; 5];
            let mut min_s = [[INF, INF], [INF, INF]];
            let mut left = 0;
            for (i, c) in s.chars().enumerate() {
                let idx = (c as u8 - b'0') as usize;
                cur_s[idx] += 1;
                let r = i + 1;
                while r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[(pre_s[x] & 1) as usize][(pre_s[y] & 1) as usize];
                    *p = min(*p, pre_s[x] - pre_s[y]);
                    let left_idx = (s.chars().nth(left).unwrap() as u8 - b'0') as usize;
                    pre_s[left_idx] += 1;
                    left += 1;
                }
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1 ^ 1) as usize][(cur_s[y] & 1) as usize]);
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap();
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();
    println!("{}", max_difference(&s, k));
}