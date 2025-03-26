use std::io::{self, Write};

// Function to calculate the smallest number with the same number of bits as `n`
fn smallest_number(n: i32) -> i32 {
    let b = (n as f64).log2() as i32 + 1; // Calculate the number of bits
    (1 << b) - 1 // Return 2^b - 1
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input as an integer
    let n: i32 = input.trim().parse().expect("Input is not a valid integer");

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Output the result
    println!("{}", result);
}