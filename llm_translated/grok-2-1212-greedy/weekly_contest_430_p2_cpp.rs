use std::cmp::max;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn answer_string(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }
        let n = s.len();
        let mut ans = String::new();
        for i in 0..n {
            let start = i;
            let end = n - max((k - 1) as usize, i);
            let substr = s[start..end].to_string();
            ans = ans.max(substr);
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input string and k
    let s = lines.next().unwrap()?;
    let k: i32 = lines.next().unwrap()?.parse().unwrap();

    // Call answer_string function
    let result = Solution::answer_string(s, k);

    // Print the result
    println!("{}", result);

    Ok(())
}