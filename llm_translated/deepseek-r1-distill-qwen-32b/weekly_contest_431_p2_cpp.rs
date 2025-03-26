use std::io;

struct Solution;

impl Solution {
    fn calculate_score(s: String) -> i64 {
        let mut stacks: Vec<Vec<i32>> = vec![Vec::new(); 26];
        let mut ans = 0;
        for (i, char) in s.chars().enumerate() {
            let c = char.to_lowercase().next().unwrap() as u8 - b'a';
            let c = c as usize;
            let opposite = 25 - c;
            if !stacks[opposite].is_empty() {
                ans += i as i64 - stacks[opposite].pop().unwrap() as i64;
            } else {
                stacks[c].push(i as i32);
            }
        }
        ans
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read input");
    let s = s.split_whitespace().next().unwrap_or_default().to_string();
    let sol = Solution;
    println!("{}", sol.calculate_score(s));
}