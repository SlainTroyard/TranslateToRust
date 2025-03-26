use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn max_difference(s: &str) -> i32 {
        let mut cnt = [0; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        let mut max1 = 0;
        let mut min0 = i32::MAX;
        for &c in &cnt {
            if c % 2 == 1 {
                max1 = max1.max(c);
            } else if c != 0 {
                min0 = min0.min(c);
            }
        }
        max1 - min0
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the input string
    let s = lines.next().unwrap()?;

    // Calculate and print the result
    let result = Solution::max_difference(&s);
    println!("{}", result);

    Ok(())
}