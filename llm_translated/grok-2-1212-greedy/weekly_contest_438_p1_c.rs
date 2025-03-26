use std::io::{self, Read};

fn has_same_digits(s: &mut [u8]) -> bool {
    let len = s.len();
    let mut i = 0;
    while i < len {
        s[i] &= 0x0f;
        i += 1;
    }
    let mut len = len;
    while len > 1 {
        len -= 1;
        i = 0;
        while i < len {
            s[i] = (s[i] as u8 + s[i + 1] as u8) % 10;
            i += 1;
        }
    }
    s[0] == s[1]
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut s: Vec<u8> = input.trim().bytes().collect();

    let result = has_same_digits(&mut s);
    println!("{}", result as i32);
    Ok(())
}