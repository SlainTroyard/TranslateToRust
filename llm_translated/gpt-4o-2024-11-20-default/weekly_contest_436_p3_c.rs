// Problem: Weekly Contest 436 Problem 3
use std::io::{self, Read};

fn count_substrings(s: &str) -> i64 {
    let mut ans = 0i64;
    let mut f = vec![vec![0; 9]; 10]; // Equivalent to f[10][9] initialized to 0

    let len = s.len();
    for i in 0..len {
        let d = (s.as_bytes()[i] - b'0') as usize; // Convert the ASCII character to its numeric value

        for m in 1..10 {
            let mut nf = vec![0; 9]; // Temporary array for the new count results
            nf[d % m] = 1; // Single digit remainder

            for rem in 0..m {
                // Update counts by appending the current digit `d` to existing remainders
                let new_rem = (rem * 10 + d) % m;
                nf[new_rem] += f[m][rem];
            }

            // Update f array
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // Count substrings where the current number `d` is divisible by itself
        ans += f[d][0];
    }

    ans
}

fn main() {
    // Read stdin (maximum size is 100,000, as inferred from the original code)
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Error reading input");

    // Trim newline and feed the string to `count_substrings` function
    let result = count_substrings(input.trim());

    // Output the result in the exact format as the original C code
    println!("{}", result);
}