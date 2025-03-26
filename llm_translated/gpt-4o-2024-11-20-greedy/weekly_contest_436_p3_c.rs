use std::io::{self, Read};

fn count_substrings(s: &str) -> i64 {
    let mut ans: i64 = 0;
    let mut f = vec![vec![0; 9]; 10]; // Initialize the count array to 0

    for ch in s.chars() {
        let d = ch.to_digit(10).unwrap() as usize; // Current digit

        for m in 1..10 {
            let mut nf = vec![0; 9]; // Temporary array to store new counts
            nf[d % m] = 1; // Single digit `d` mod `m`

            for rem in 0..m {
                // Update counts: Add current digit `d` to existing remainder `rem`
                nf[(rem * 10 + d) % m] += f[m][rem];
            }

            // Update `f` array
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // If the current digit is divisible by itself, increase the result count
        ans += f[d][0];
    }

    ans
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Trim the input to remove any trailing newlines or spaces
    let s = input.trim();

    // Call the function to compute the result
    let result = count_substrings(s);

    // Print the result to stdout
    println!("{}", result);
}