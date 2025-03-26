// Problem: Weekly Contest 435 Problem 1

use std::io;

fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    for &byte in s.as_bytes() {
        cnt[(byte - b'a') as usize] += 1;
    }

    let mut c1 = 0;
    let mut c2 = 100;

    for i in 0..26 {
        if cnt[i] & 1 != 0 {
            c1 = c1.max(cnt[i]);
        } else if cnt[i] != 0 {
            c2 = c2.min(cnt[i]);
        }
    }

    c1 - c2
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    println!("{}", max_difference(s));
}