// Problem: Weekly Contest 435 Problem 1
use std::io;

struct Solution;

impl Solution {
    fn max_difference(&self, s: String) -> i32 {
        let mut cnt = [0; 26];
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
        }

        let mut max1 = 0;
        let mut min0 = i32::MAX;

        for &c in cnt.iter() {
            if c % 2 != 0 {
                if c > max1 {
                    max1 = c;
                }
            } else if c > 0 {
                if c < min0 {
                    min0 = c;
                }
            }
        }

        max1 - min0
    }
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let s = s.trim().to_string();
    let sol = Solution;
    println!("{}", sol.max_difference(s));
    Ok(())
}