use std::io;
use std::io::Read;
use std::io::Write;
use std::f64;

struct Solution {}

impl Solution {
    fn smallest_number(&self, n: i32) -> i32 {
        let b = (n as f64).log2().floor() as i32 + 1; // Calculate the number of bits
        (1 << b) - 1 // Return 2^b - 1
    }
}

fn main() {
    let solution = Solution {};
    let mut n_str = String::new();

    // Input
    io::stdin().read_to_string(&mut n_str).expect("Failed to read line");
    let n: i32 = n_str.trim().parse().expect("Failed to parse integer");

    // Calculate the smallest number
    let result = solution.smallest_number(n);

    // Output
    io::stdout().write_all(format!("{}\n", result).as_bytes()).expect("Failed to write");
}