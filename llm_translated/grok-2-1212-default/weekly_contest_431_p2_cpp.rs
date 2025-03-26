use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn calculate_score(s: &str) -> i64 {
        let mut stk: [Vec<i32>; 26] = Default::default();
        let mut ans: i64 = 0;
        for (i, &c) in s.as_bytes().iter().enumerate() {
            let c = (c - b'a') as usize;
            if let Some(&top) = stk[25 - c].last() {
                ans += i as i64 - top as i64;
                stk[25 - c].pop();
            } else {
                stk[c].push(i as i32);
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input string
    let s = lines.next().unwrap()?.trim().to_string();

    // Calculate and print the result
    let result = Solution::calculate_score(&s);
    println!("{}", result);

    Ok(())
}