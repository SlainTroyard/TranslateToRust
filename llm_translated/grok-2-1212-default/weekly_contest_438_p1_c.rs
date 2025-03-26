use std::io::{self, Read};

fn has_same_digits(s: &mut [u8]) -> bool {
    // Convert each character to its numeric value
    for c in s.iter_mut() {
        *c &= 0x0f;
    }

    // Perform the reduction operation
    let mut len = s.len();
    while len > 1 {
        for i in 0..len - 1 {
            s[i] = (s[i] + s[i + 1]) % 10;
        }
        len -= 1;
    }

    // Compare the final two digits
    s[0] == s[1]
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Process each line of input
    for line in input.lines() {
        let mut s: Vec<u8> = line.trim().bytes().collect();
        let result = has_same_digits(&mut s);
        println!("{}", if result { 1 } else { 0 });
    }

    Ok(())
}