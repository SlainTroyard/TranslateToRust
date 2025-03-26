use std::io;

/// Calculate the smallest number with the same number of bits as `n`.
fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits by finding the log base 2 of `n`, adding 1
    let b = ((n as f64).log2().ceil() as i32) + 1; 
    // Return 2^b - 1
    (1 << b) - 1
}

fn main() {
    // Create a buffer for stdin and read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input as an integer
    let n: i32 = input.trim().parse().expect("Failed to parse integer");

    // Calculate the result
    let result = smallest_number(n);

    // Print the result
    println!("{}", result);
}