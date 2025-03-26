use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn calculate_score(s: String) -> i64 {
        let mut stk: [Vec<i32>; 26] = Default::default();
        let mut ans: i64 = 0;
        for (i, c) in s.chars().enumerate() {
            let c = c as i32 - 'a' as i32;
            if let Some(&top) = stk[25 - c as usize].last() {
                ans += i as i64 - top as i64;
                stk[25 - c as usize].pop();
            } else {
                stk[c as usize].push(i as i32);
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input string
    if let Some(Ok(s)) = lines.next() {
        let result = Solution::calculate_score(s);
        println!("{}", result);
    }

    Ok(())
}