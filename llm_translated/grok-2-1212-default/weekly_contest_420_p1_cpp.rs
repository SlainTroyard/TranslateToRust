use std::io::{self, Write};

struct Solution;

impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut s = String::new();
        for c in target.chars() {
            s.push('a'); // Placeholder
            for j in 'a'..=c {
                s.pop();
                s.push(j);
                ans.push(s.clone());
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let target = input.trim().to_string();

    let ans = Solution::string_sequence(target);
    for s in ans {
        print!("{} ", s);
    }
    println!();

    Ok(())
}