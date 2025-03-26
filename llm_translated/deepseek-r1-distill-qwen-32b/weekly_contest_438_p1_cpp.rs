use std::io;

// Problem: Weekly Contest 438 Problem 1
// This Rust code is the idiomatic translation of the provided C++ solution.

fn has_same_digits(mut s: String) -> bool {
    while s.len() > 2 {
        let mut t = String::new();
        for i in 0..s.len() - 1 {
            // Convert characters to their numeric values
            let x = s.as_bytes()[i] as i32 - '0' as i32;
            let y = s.as_bytes()[i + 1] as i32 - '0' as i32;
            
            // Compute the sum modulo 10 and append as a new character
            let sum = (x + y) % 10;
            t.push((sum as u8 + '0' as u8) as char);
        }
        s = t;
    }
    // Ensure the string has at least two characters before comparison
    s.len() >= 2 && s.as_bytes()[0] == s.as_bytes()[1]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let s = input.trim().to_string();
    let result = has_same_digits(s);
    println!("{}", result);
    Ok(())
}