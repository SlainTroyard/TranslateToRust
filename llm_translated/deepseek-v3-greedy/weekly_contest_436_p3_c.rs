use std::io::{self, Write};

fn count_substrings(s: &str) -> i64 {
    let mut ans: i64 = 0;
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

            // Update f array
            for rem in 0..m {
                f[m][rem] = nf[rem];
            }
        }

        // If the current digit is divisible by itself, increment the result count
        ans += f[d][0] as i64;
    }

    ans
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let s = input.trim(); // Remove any trailing newline

    // Call the function to compute the result
    let result = count_substrings(s);

    // Output the result
    println!("{}", result);
}