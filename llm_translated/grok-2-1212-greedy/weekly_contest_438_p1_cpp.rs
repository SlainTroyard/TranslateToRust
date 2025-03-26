use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn has_same_digits(s: String) -> bool {
        let mut s = s;
        while s.len() > 2 {
            let mut t = String::new();
            for i in 0..s.len() - 1 {
                let x = s.chars().nth(i).unwrap() as u8 - b'0';
                let y = s.chars().nth(i + 1).unwrap() as u8 - b'0';
                t.push((((x + y) % 10) + b'0') as char);
            }
            s = t;
        }
        s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the input string
    let s = lines.next().unwrap()?.trim().to_string();

    // Process the input and get the result
    let result = Solution::has_same_digits(s);

    // Print the result
    println!("{}", result);

    Ok(())
}