use std::io::{self, Read};

// Define the Solution struct (empty struct as placeholder)
struct Solution;

impl Solution {
    // This function calculates the smallest number that has exactly the required number of bits
    // The original C++ code does:
    //   int b = log2(n) + 1;
    //   return (1 << b) - 1;
    // In Rust, we use f64's log2 to compute the number of bits, then use bit shifting.
    pub fn smallest_number(n: i32) -> i32 {
        // Convert n to f64 and apply log2, then cast to i32.
        // This effectively computes floor(log2(n)) + 1.
        let b = (n as f64).log2() as i32 + 1;
        // Compute (1 << b) - 1, which is 2^b - 1
        (1 << b) - 1
    }
}

fn main() {
    // Read entire input from stdin.
    // The original C++ code uses "cin >> n" which reads a token from a stream of whitespace
    // This handles inputs with multiple lines or multiple values per line.
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    // Split the input into tokens separated by whitespace.
    let mut tokens = input.split_whitespace();

    // Extract the first token as an integer.
    // If the token is missing or cannot be parsed to i32, print an error and exit.
    let n: i32 = match tokens.next() {
        Some(token) => token.parse().expect("Failed to parse integer"),
        None => {
            eprintln!("No input provided");
            return;
        }
    };

    // Calculate the smallest number using the provided algorithm.
    let result = Solution::smallest_number(n);

    // Output the result followed by a newline, matching the original format.
    println!("{}", result);
}