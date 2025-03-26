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
                t.push(char::from_digit((x + y) % 10, 10).unwrap());
            }
            s = t;
        }
        s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap()
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let s = lines.next().unwrap()?.trim().to_string();

    // Process and output result
    let sol = Solution;
    println!("{}", sol.has_same_digits(s));

    Ok(())
}