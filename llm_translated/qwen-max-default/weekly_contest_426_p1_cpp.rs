// Problem: Weekly Contest 426 Problem 1

use std::io;

fn main() {
    // Read the input number from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please type a number!");

    // Calculate the smallest number
    let result = smallest_number(n);

    // Output the result to stdout
    println!("{}", result);
}

/// Calculate the smallest number with the same number of bits as `n`.
///
/// # Arguments
///
/// * `n` - An integer.
///
/// # Returns
///
/// * The smallest number with the same number of bits as `n`.
fn smallest_number(n: i32) -> i32 {
    let b = (n as f64).log2().ceil() as u32;  // Calculate the number of bits
    (1 << b) - 1  // Return 2^b - 1
}