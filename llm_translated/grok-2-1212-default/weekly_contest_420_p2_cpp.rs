use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i64 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];

        for c in s.chars() {
            let index = (c as u8 - b'a') as usize;
            cnt[index] += 1;

            while cnt[index] >= k as usize {
                cnt[(s.chars().nth(left).unwrap() as u8 - b'a') as usize] -= 1;
                left += 1;
            }

            ans += left as i64;
        }

        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?;
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Process and output result
    let sol = Solution;
    println!("{}", sol.number_of_substrings(s, k));

    Ok(())
}