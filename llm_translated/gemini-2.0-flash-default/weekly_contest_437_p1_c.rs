use std::io;
use std::io::Read;

fn has_special_substring(s: &str, k: i32) -> bool {
    let length = s.len();
    let mut consecutive = 0;
    let chars: Vec<char> = s.chars().collect(); // Convert to vector of chars for easy indexing
    for i in 0..length {
        consecutive += 1;
        if i == length - 1 || chars[i] != chars[i + 1] {
            if consecutive == k {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    false
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut iter = input.split_whitespace();
    let s = iter.next().unwrap(); //unwrap is safe because the input should always have a string
    let k: i32 = iter.next().unwrap().parse().unwrap(); //unwrap is safe because the input should always have an integer
    let result = has_special_substring(s, k);
    println!("{}", if result { 1 } else { 0 });

    Ok(())
}