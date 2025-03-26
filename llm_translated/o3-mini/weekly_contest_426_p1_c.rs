use std::io::{self, Read};

fn smallest_number(n: i32) -> i32 {
    // Calculate the number of bits in the binary representation of n:
    // (int)(log2(n)) + 1 is equivalent to flooring the logarithm base 2 and adding one.
    let b = (n as f64).log2().floor() as i32 + 1;
    // Return 2^b - 1, which is a number represented by b ones in binary
    (1 << b) - 1
}

fn main() -> io::Result<()> {
    // Read complete input from stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Split the input by whitespace and take the first token as n
    // If no valid integer is found, print an error and exit.
    let n: i32 = match buffer.split_whitespace().next() {
        Some(token) => token.parse().unwrap_or_else(|_| {
            eprintln!("Error: Could not parse '{}' as an integer.", token);
            std::process::exit(1);
        }),
        None => {
            eprintln!("Error: No input provided.");
            std::process::exit(1);
        }
    };

    // Compute the smallest number with the same number of bits as n
    let result = smallest_number(n);

    // Print the result (with a newline as in the original C code)
    println!("{}", result);

    Ok(())
}