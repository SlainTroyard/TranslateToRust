// Problem: Weekly Contest 426 Problem 1
use std::io;

fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits
    let b = (n as f64).log2().floor() as u32 + 1;
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() {
    // Input the number
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please type a number!");

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Output the result
    println!("{}", result);
}