use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut cnt = [0; 26];
        for b in s.chars() {
            cnt[(b as u8 - b'a') as usize] += 1;
        }

        let mut max1 = 0;
        let mut min0 = i32::MAX;
        for &c in cnt.iter() {
            if c % 2 == 1 {
                max1 = max1.max(c);
            } else if c > 0 {
                min0 = min0.min(c);
            }
        }
        max1 - min0
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input string
    if let Some(Ok(s)) = lines.next() {
        let result = Solution::max_difference(s);
        println!("{}", result);
    }

    Ok(())
}