/// LeetCode Weekly Contest 426 Problem 1
///
/// This program reads an integer `n` from standard input, calculates the
/// smallest number that has the same number of bits set (in the binary
/// representation of `n`), and prints the result.
///
/// The logic is:
///   b = floor(log2(n)) + 1      // number of bits
///   result = (1 << b) - 1       // 2^b - 1
///
/// The input handling mirrors the C code's single integer input with `scanf("%d", &n)`.
/// Any extra data in the input is ignored. The output format is identical to the C code.

use std::error::Error;
use std::io::{self, BufRead};

/// Returns the smallest integer that has the same number of bits as `n`.
fn smallest_number(n: i32) -> i32 {
    // number of bits is floor(log2(n)) + 1
    let bits = ((n as f64).log2().floor() as i32) + 1;
    (1 << bits) - 1
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read one line from stdin, parse an integer from it
    if let Some(Ok(line)) = lines.next() {
        let n: i32 = line.trim().parse()?;
        // Calculate the smallest number with the same number of bits
        let result = smallest_number(n);
        // Print the result
        println!("{}", result);
    }

    Ok(())
}