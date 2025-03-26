// Problem: Weekly Contest 420 Problem 1
use std::io;

struct Solution {}

impl Solution {
    fn string_sequence(target: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut s: String = String::new();
        for c in target.chars() {
            s.push('a'); // Placeholder
            for j_char_code in b'a'..=c as u8 {
                let j = j_char_code as char;
                if let Some(last_char) = s.chars().last_mut() {
                    *last_char = j;
                }
                ans.push(s.clone());
            }
        }
        ans
    }
}

fn main() {
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target = target.trim(); // Remove trailing newline

    let solution = Solution {};
    let ans = solution.string_sequence(target.to_string());

    for s in ans {
        print!("{} ", s);
    }
    println!();
}