use std::io::{self, Write};

struct Solution;

impl Solution {
    fn has_same_digits(s: String) -> bool {
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().to_string(); // Trim any trailing newline characters
    
    let sol = Solution;
    let result = sol.has_same_digits(input);
    println!("{}", result);
}