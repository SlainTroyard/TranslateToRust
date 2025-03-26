// Problem: Weekly Contest 417 Problem 1

use std::io::{self, BufRead};

fn kth_character(k: i64) -> char {
    // Convert k to u64 for popcount
    let k_u64 = (k - 1) as u64;
    // Calculate the number of set bits in k-1
    let set_bits = k_u64.count_ones();
    // Return the character 'a' offset by the number of set bits
    (b'a' + set_bits as u8) as char
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line as an i64
    let k: i64 = lines.next().unwrap()?.trim().parse().unwrap();

    // Calculate and print the result
    let result = kth_character(k);
    println!("{}", result);

    Ok(())
}