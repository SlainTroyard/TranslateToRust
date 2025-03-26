use std::io;

struct Solution;

impl Solution {
    pub fn kth_character(k: i64) -> char {
        // Calculate m = k - 1 and cast to u32 to mimic the original C++ __builtin_popcount behavior
        let m = k - 1;
        // Count the number of set bits in the lower 32 bits (m as u32)
        let count = (m as u32).count_ones() as u8;
        // Convert 'a' to byte, add the count, and cast back to char
        (b'a' + count) as char
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // Parse the first whitespace-separated token as i64, default to 0 on failure
    let k: i64 = input
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    println!("{}", Solution::kth_character(k));
}