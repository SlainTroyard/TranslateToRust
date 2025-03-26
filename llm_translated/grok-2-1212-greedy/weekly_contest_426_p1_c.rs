use std::io::{self, Read};

/// Calculates the smallest number with the same number of bits as `n`.
fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits
    let b = (n as f64).log2().floor() as i32 + 1;
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse the input number
    let n: i32 = input.trim().parse().expect("Failed to parse input");

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Output the result to stdout
    println!("{}", result);

    Ok(())
}