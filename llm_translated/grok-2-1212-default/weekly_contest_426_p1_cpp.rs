use std::io::{self, BufRead};

struct Solution;

impl Solution {
    /// Calculates the smallest number that is greater than or equal to n
    /// and is one less than a power of 2.
    ///
    /// # Arguments
    ///
    /// * `n` - The input number
    ///
    /// # Returns
    ///
    /// The smallest number that satisfies the condition
    fn smallest_number(n: i32) -> i32 {
        let b = (n as f64).log2().floor() as u32 + 1;  // Calculate the number of bits
        (1 << b) - 1                                  // Return 2^b - 1
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