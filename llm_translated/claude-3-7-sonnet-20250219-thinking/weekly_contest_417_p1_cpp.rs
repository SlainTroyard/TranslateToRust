use std::io;

struct Solution {}

impl Solution {
    fn kth_character(&self, k: u64) -> char {
        // Equivalent to __builtin_popcount in C++
        // Count the number of 1 bits in the binary representation of k-1
        let count = (k - 1).count_ones() as u8;
        (b'a' + count) as char
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse input to u64
    let k: u64 = input.trim().parse().expect("Invalid input");
    
    // Create Solution instance and call method
    let s = Solution {};
    
    // Print result to stdout with newline (matching C++ endl)
    println!("{}", s.kth_character(k));
}