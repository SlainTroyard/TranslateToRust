use std::cmp::{max, min};
use std::io::{self, Write};

struct Solution;

impl Solution {
    fn max_difference(s: String) -> i32 {
        let mut cnt = [0; 26];
        for b in s.chars() {
            cnt[(b as u8 - b'a') as usize] += 1;
        }

        let mut max1 = 0;
        let mut min0 = i32::MAX;
        for &c in cnt.iter() {
            if c % 2 == 1 {
                max1 = max(max1, c);
            } else if c != 0 {
                min0 = min(min0, c);
            }
        }
        max1 - min0
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim().to_string();
    let result = Solution::max_difference(s);
    println!("{}", result);
}