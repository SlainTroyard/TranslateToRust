use std::collections::VecDeque;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        let mut stk: Vec<VecDeque<usize>> = vec![VecDeque::new(); 26];
        let mut ans: i64 = 0;
        
        for (i, ch) in s.chars().enumerate() {
            let c = (ch as u8 - b'a') as usize;
            if let Some(&top) = stk[25 - c].back() {
                ans += (i - top) as i64;
                stk[25 - c].pop_back();
            } else {
                stk[c].push_back(i);
            }
        }
        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    if let Some(Ok(s)) = lines.next() {
        let sol = Solution;
        println!("{}", sol.calculate_score(s));
    }
}