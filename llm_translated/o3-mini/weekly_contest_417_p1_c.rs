use std::io::{self, BufRead};

/// Computes the kth character according to the problem logic.
/// It builds an array `l` incrementally and returns the character:
///   'a' + (l[k-1] % 26)
fn kth_character(k: usize) -> char {
    // Allocate vector with k+1 elements initialized to 0
    let mut l = vec![0; k + 1];
    let mut a: usize = 0;

    // Outer loop, guaranteed to eventually hit k - 1 index.
    loop {
        // j = 1 << a, i.e., 2^a
        let j = 1 << a;
        // For every index in the current block [0, 1<<a)
        for i in 0..j {
            let idx = i + j;
            // Extend the sequence by setting l[idx] = l[i] + 1
            l[idx] = l[i] + 1;

            // Check if we have filled at least up to index k-1
            if idx >= k - 1 {
                // Return character: 'a' + (l[k-1] % 26)
                return (b'a' + (l[k - 1] % 26) as u8) as char;
            }
        }
        a += 1;
    }
}

fn main() {
    // Use a buffered reader to handle input values from STDIN.
    let stdin = io::stdin();
    let mut k: Option<usize> = None;

    // Read lines from standard input. The input can be in multiple lines or multiple tokens per line.
    for line in stdin.lock().lines() {
        match line {
            Ok(text) => {
                // Split the line into tokens on whitespace
                for token in text.split_whitespace() {
                    if let Ok(num) = token.parse::<usize>() {
                        k = Some(num);
                        // Once we find the first number, stop further reading
                        break;
                    }
                }
                if k.is_some() {
                    break;
                }
            }
            Err(err) => {
                // If there is any error reading input, report it and exit.
                eprintln!("Error reading input: {}", err);
                std::process::exit(1);
            }
        }
    }

    // Ensure that we obtained a value for k, or exit if not.
    let k = k.expect("Input integer k is required");

    // Calculate the kth character using the provided algorithm.
    let result = kth_character(k);

    // Print the result to standard output following the expected format.
    println!("{}", result);
}