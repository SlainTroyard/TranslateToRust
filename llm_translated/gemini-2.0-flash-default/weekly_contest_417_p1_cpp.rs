// Problem: Weekly Contest 417 Problem 1
use std::io;

struct Solution {}

impl Solution {
    pub fn kth_character(&self, k: i64) -> char {
        // __builtin_popcount is equivalent to count_ones in Rust
        let count = (k - 1).count_ones();
        (b'a' + count as u8) as char
    }
}

fn main() {
    let mut k = String::new();
    io::stdin()
        .read_line(&mut k)
        .expect("Failed to read line");

    let k: i64 = k.trim().parse().expect("Please type a number!");

    let s = Solution {};
    println!("{}", s.kth_character(k));
}