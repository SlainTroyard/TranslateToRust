// Problem: Weekly Contest 436 Problem 3
// Translated from C to Rust

use std::io::{self, Write};
use std::process;

/// Counts the number of valid substrings according to the original C logic.
fn count_substrings(s: &str) -> i64 {
    let mut ans: i64 = 0;
    // f[m][rem]: count of substrings that have remainder rem when taken mod m
    let mut f: [[i64; 9]; 10] = [[0; 9]; 10];

    // Process each character of the input string
    let chars: Vec<char> = s.chars().collect();
    for &ch in &chars {
        // Convert character to digit
        let d = ch.to_digit(10).unwrap() as usize;
        
        // Update counts for each modulus 1..9
        for m in 1..10 {
            // Temporary array to store new counts for this coordinate
            let mut nf = [0i64; 9];
            
            // Single digit's remainder
            nf[d % m] = 1;

            // Update nf for all existing remainders in f[m]
            for rem in 0..m {
                let new_rem = (rem * 10 + d) % m;
                nf[new_rem] += f[m][rem];
            }

            // Write updated counts back into f[m]
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // The current digit by itself, considered under d as modulus,
        // contributes to substrings if the remainder is 0
        ans += f[d][0];
    }

    ans
}

fn main() {
    let mut buffer = String::new();

    // Read a single token from stdin (similar to scanf("%s", s) in C)
    match io::stdin().read_line(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                eprintln!("Error reading input");
                process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("Error reading input");
            process::exit(1);
        }
    };

    // Split by whitespace and take the first part (as scanf would do)
    let token = buffer.split_whitespace().next();
    let s = match token {
        Some(t) => t,
        None => {
            eprintln!("Error reading input");
            process::exit(1);
        }
    };

    // Compute the result using the translated function
    let result = count_substrings(s);

    // Print the result in the same format
    println!("{}", result);
}