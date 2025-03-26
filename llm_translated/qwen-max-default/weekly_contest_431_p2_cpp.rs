use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn calculate_score(s: &str) -> i64 {
        let mut stk = vec![Vec::new(); 26];
        let mut ans = 0;

        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 'a' as usize;
            if !stk[25 - idx].is_empty() {
                ans += i as i64 - *stk[25 - idx].last().unwrap();
                stk[25 - idx].pop();
            } else {
                stk[idx].push(i);
            }
        }

        ans
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the input string
    let s = lines.next().unwrap().expect("Failed to read input");

    // Create an instance of Solution and calculate the score
    let sol = Solution;
    let result = sol.calculate_score(&s);

    // Print the result
    println!("{}", result);
}