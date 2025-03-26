use std::io;
use std::io::BufRead;

fn smallest_number(n: i32) -> i32 {
    if n <= 0 {
        return 0; // Handle non-positive input as the original C code might have undefined behavior for log2(n) when n <= 0. For practical purposes, if n is 0, the smallest number with the same number of bits could be considered 0.
    }
    let b = 32 - n.leading_zeros(); // Calculate the number of bits required to represent n
    (1 << b) - 1                  // Return 2^b - 1
}

fn main() {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    let mut input_line = String::new();

    if reader.read_line(&mut input_line).is_ok() {
        let trimmed_line = input_line.trim();
        if let Ok(n) = trimmed_line.parse::<i32>() {
            let result = smallest_number(n); // Calculate the smallest number with the same number of bits as `n`
            println!("{}", result);         // Output the result
        } else {
            eprintln!("Invalid input: Please enter an integer."); // Error message for invalid input
        }
    } else {
        eprintln!("Failed to read input."); // Error message if reading input fails
    }
}