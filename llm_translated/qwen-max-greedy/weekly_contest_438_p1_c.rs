// Problem: Weekly Contest 438 Problem 1

use std::io;

fn has_same_digits(s: &mut [u8]) -> bool {
    let len = s.len();
    for i in 0..len {
        s[i] &= 0x0f; // Mask the upper 4 bits to keep only the lower 4 bits
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

    let mut s: Vec<u8> = input.trim().bytes().collect();
    let result = has_same_digits(&mut s);

    println!("{}", if result { 1 } else { 0 });
    Ok(())
}