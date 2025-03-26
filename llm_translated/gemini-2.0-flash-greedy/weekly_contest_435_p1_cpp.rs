use std::io;
use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_difference(&self, s: String) -> i32 {
        let mut cnt = [0; 26];
        for b in s.chars() {
            cnt[(b as u8 - b'a') as usize] += 1;
        }

        let mut max1 = 0;
        let mut min0 = i32::MAX;

        for c in cnt.iter() {
            if c % 2 != 0 {
                max1 = max(max1, *c);
            } else if *c != 0 {
                min0 = min(min0, *c);
            }
        }

        max1 - min0
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim().to_string();

    let sol = Solution {};
    println!("{}", sol.max_difference(s));
}