use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 26];

        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
            while cnt[(c as u8 - b'a') as usize] >= k as usize {
                cnt[(s.chars().nth(left).unwrap() as u8 - b'a') as usize] -= 1;
                left += 1;
            }
            ans += left as i32;
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

    // Process and output result
    let sol = Solution;
    println!("{}", sol.number_of_substrings(s, k));

    Ok(())
}