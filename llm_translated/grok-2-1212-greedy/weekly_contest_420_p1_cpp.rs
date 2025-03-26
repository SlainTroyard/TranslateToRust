use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn string_sequence(target: String) -> Vec<String> {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the target string
    let target = lines.next().unwrap()?.trim().to_string();

    let ans = Solution::string_sequence(target);

    // Print the result
    for s in ans {
        print!("{} ", s);
    }
    println!();

    Ok(())
}