use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    fn has_same_digits(&self, s: String) -> bool {
        let mut s = s;
        while s.len() > 2 {
            let mut t = String::new();
            for i in 0..s.len() - 1 {
                let x = s.chars().nth(i).unwrap() as u8 - b'0';
                let y = s.chars().nth(i + 1).unwrap() as u8 - b'0';
                t.push(((x + y) % 10 + b'0') as char);
            }
            s = t;
        }
        s.chars().nth(0) == s.chars().nth(1)
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string(); // Remove trailing newline

    let sol = Solution {};
    println!("{}", sol.has_same_digits(s));
}