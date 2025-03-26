// Problem: Weekly Contest 426 Problem 1

use std::io;

fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits needed to represent `n`
    let b = (n as f64).log2().ceil() as u32;
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the input number from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Parse the input string to an integer
    let n: i32 = input.trim().parse()?;
    
    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);
    
    // Output the result to stdout
    println!("{}", result);

    Ok(())
}