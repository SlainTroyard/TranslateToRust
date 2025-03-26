use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_difference(s: &str, k: i32) -> i32 {
        const INF: i32 = i32::MAX / 2;
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
                for (i, &c) in s.as_bytes().iter().enumerate() {
                    cur_s[(c - b'0') as usize] += 1;
                    let r = i + 1;
                    while r as i32 - left as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let p = &mut min_s[(pre_s[x] & 1) as usize][(pre_s[y] & 1) as usize];
                        *p = (*p).min(pre_s[x] - pre_s[y]);
                        pre_s[(s.as_bytes()[left] - b'0') as usize] += 1;
                        left += 1;
                    }
                    ans = ans.max(cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1 ^ 1) as usize][(cur_s[y] & 1) as usize]);
                }
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?.trim().to_string();
    let k: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Calculate result
    let result = Solution::max_difference(&s, k);

    // Print output
    println!("{}", result);

    Ok(())
}