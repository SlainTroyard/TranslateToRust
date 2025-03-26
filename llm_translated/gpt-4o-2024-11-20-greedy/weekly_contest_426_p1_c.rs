// Problem: Weekly Contest 426 Problem 1
use std::io;

fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits required to represent `n`
    let b = (n as f64).log2().floor() as i32 + 1;
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() {
    // Create a buffer to read input
    let mut input = String::new();

    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input as an integer
    let n: i32 = input.trim().parse().expect("Invalid input");

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Print the result to stdout
    println!("{}", result);
}