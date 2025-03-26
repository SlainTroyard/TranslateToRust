// Problem: Weekly Contest 426 Problem 1
use std::io;
use std::io::Read;
use std::io::Write;
use std::cmp;

fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits needed to represent n
    let b = (n as f64).log2().floor() as i32 + 1;
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let n: i32 = input.trim().parse()?;

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Output the result
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result)?;

    Ok(())
}