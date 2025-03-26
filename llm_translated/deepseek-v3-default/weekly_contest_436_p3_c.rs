use std::io::{self, Write};

fn count_substrings(s: &str) -> i64 {
    let mut ans = 0;
    let mut f = [[0; 9]; 10]; // Initialize the counting array to 0

    for c in s.chars() {
        let d = c.to_digit(10).unwrap() as usize; // Current digit

        for m in 1..10 {
            let mut nf = [0; 9]; // Temporary array to hold new counts
            nf[d % m] = 1; // Single digit d modulo m

            for rem in 0..m {
                // Update counts: append current digit d to existing remainder rem
                nf[(rem * 10 + d) % m] += f[m][rem];
            }

            // Update the f array
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // If the current digit is divisible by itself, increment the result
        ans += f[d][0];
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Remove any trailing newline or whitespace

    let result = count_substrings(s);

    println!("{}", result);
}