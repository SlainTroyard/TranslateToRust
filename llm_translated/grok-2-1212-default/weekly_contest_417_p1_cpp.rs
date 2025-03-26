// Problem: Weekly Contest 417 Problem 1

use std::io::{self, BufRead};

fn kth_character(k: i64) -> char {
    // Convert k to u64 for popcount
    let k_u64 = k as u64;
    // Calculate the number of set bits in k - 1
    let set_bits = k_u64.wrapping_sub(1).count_ones();
    // Return the character 'a' offset by the number of set bits
    (b'a' + set_bits as u8) as char
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let k: i64 = line?.trim().parse().expect("Failed to parse input");
        // Calculate and print the kth character
        println!("{}", kth_character(k));
    }
    Ok(())
}