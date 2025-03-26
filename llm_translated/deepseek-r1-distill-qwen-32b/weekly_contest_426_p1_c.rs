// Problem: Weekly Contest 426 Problem 1

fn smallest_number(n: i32) -> i32 {
    let bits = n.bit_length();
    (1 << bits) - 1
}

fn main() {
    use std::io;

    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the integer
    let n: i32 = input.trim().parse().expect("Please enter a valid integer");

    // Calculate the smallest number with the same number of bits as `n`
    let result = smallest_number(n);

    // Output the result
    println!("{}", result);
}