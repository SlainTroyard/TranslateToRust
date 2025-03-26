// Problem: Weekly Contest 431 Problem 2
use std::io;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn calculate_score(s: &str) -> i64 {
        let mut stk: Vec<VecDeque<usize>> = vec![VecDeque::new(); 26];
        let mut ans: i64 = 0;

        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 'a' as usize;
            if let Some(&top) = stk[25 - idx].front() {
                ans += (i - top) as i64;
                stk[25 - idx].pop_front();
            } else {
                stk[idx].push_back(i);
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim(); // Trim the newline character

    let sol = Solution;
    let result = sol.calculate_score(input);
    println!("{}", result);
}