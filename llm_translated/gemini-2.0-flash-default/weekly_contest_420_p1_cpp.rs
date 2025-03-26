use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn string_sequence(&self, target: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut s: String = String::new();
        for c in target.chars() {
            s.push('a'); //占位
            for j in b'a'..=c as u8 {
                s.pop();
                s.push(j as char);
                ans.push(s.clone());
            }
        }
        ans
    }
}

fn main() {
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Failed to read line");
    let target = target.trim().to_string(); // Remove trailing newline

    let solution = Solution {};
    let ans = solution.string_sequence(target);

    for s in ans {
        print!("{} ", s);
    }
    println!();
}