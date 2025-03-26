// Problem: Weekly Contest 438 Problem 1
use std::io;

fn has_same_digits(s: &mut [u8]) -> bool {
    let len = s.len();
    for i in 0..len {
        s[i] &= 0x0f; // Mask the higher 4 bits, keeping only the lower 4 bits (0-9)
    }

    let mut len = len;
    while len > 1 {
        for i in 0..len - 1 {
            s[i] = (s[i] + s[i + 1]) % 10;
        }
        len -= 1;
    }

    s[0] == s[1]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim(); // Remove any trailing newline or whitespace

    let mut s: Vec<u8> = input.bytes().collect();
    let result = has_same_digits(&mut s);

    println!("{}", if result { 1 } else { 0 });
    Ok(())
}