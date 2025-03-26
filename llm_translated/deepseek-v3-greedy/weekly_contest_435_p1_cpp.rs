use std::io;
use std::cmp::{max, min};

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    for b in s.chars() {
        cnt[(b as u8 - b'a') as usize] += 1;
    }

    let mut max1 = 0;
    let mut min0 = i32::MAX;
    for &c in cnt.iter() {
        if c % 2 == 1 {
            max1 = max(max1, c);
        } else if c != 0 {
            min0 = min(min0, c);
        }
    }
    max1 - min0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Remove any trailing newline characters

    let result = max_difference(s);
    println!("{}", result);
}