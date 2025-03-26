use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        // Calculate the number of bits
        let b = (n as f64).log2().floor() as i32 + 1;
        // Return 2^b - 1
        (1 << b) - 1
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Input
    let n: i32 = lines.next().unwrap()?.trim().parse().unwrap();

    // Calculate the smallest number
    let result = Solution::smallest_number(n);

    // Output
    println!("{}", result);

    Ok(())
}