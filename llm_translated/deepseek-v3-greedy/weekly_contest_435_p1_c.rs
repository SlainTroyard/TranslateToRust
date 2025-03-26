use std::io::{self, Write};

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    for c in s.chars() {
        cnt[(c as u8 - b'a') as usize] += 1;
    }

    let mut c1 = 0;
    let mut c2 = 100;

    for &count in cnt.iter() {
        if count % 2 == 1 {
            c1 = c1.max(count);
        } else if count != 0 {
            c2 = c2.min(count);
        }
    }

    c1 - c2
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim(); // Remove any trailing newline

    let result = max_difference(input);
    println!("{}", result);
}