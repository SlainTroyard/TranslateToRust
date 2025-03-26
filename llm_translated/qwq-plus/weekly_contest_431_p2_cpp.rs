use std::io;

struct Solution;

impl Solution {
    fn calculate_score(s: String) -> i64 {
        let mut stacks: Vec<Vec<usize>> = vec![Vec::new(); 26];
        let mut ans: i64 = 0;
        for (i, c) in s.bytes().enumerate() {
            let c_val = (c - b'a') as usize;
            let opposite = 25 - c_val;
            if let Some(top) = stacks[opposite].pop() {
                ans += (i as i64) - (top as i64);
            } else {
                stacks[c_val].push(i);
            }
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let s = input.trim().to_string();
    let sol = Solution;
    println!("{}", sol.calculate_score(s));
}