use std::io;

/// Calculates the smallest number with the same number of bits as `n`.
/// The number of bits is determined by finding the highest set bit's position.
/// The result is (2^b - 1), where b is the number of bits.
fn smallest_number(n: u32) -> u32 {
    let b = 32 - n.leading_zeros();
    (1 << b) - 1
}

fn main() {
    let mut input = String::new();
    // Read the entire line from stdin
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    // Parse the first integer from the input
    let n: u32 = input.trim().parse()
        .expect("Input is not a valid integer");
    let result = smallest_number(n);
    // Output the result with a newline
    println!("{}", result);
}