// Problem: Weekly Contest 426 Problem 1
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;
use std::str::FromStr;

fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits
    let b = (n as f64).log2().floor() as i32 + 1;
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .unwrap_or_else(|err| {
            eprintln!("Error reading input: {}", err);
            process::exit(1);
        });

    let n: i32 = input
        .trim()
        .parse()
        .unwrap_or_else(|err| {
            eprintln!("Error parsing input: {}", err);
            process::exit(1);
        });

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Output the result
    println!("{}", result);
}