use std::io::{self, BufRead, Write};

fn count_substrings(s: &str) -> i64 {
    // "ans" holds the final count.
    let mut ans: i64 = 0;
    // f is a 10x9 array of counts (we only use f[1] to f[9] in updates).
    // For each m (1 <= m <= 9), f[m][r] stores the count of substrings (or more precisely,
    // partial accumulations) that currently leave remainder r modulo m.
    let mut f = [[0; 9]; 10]; // indexed 0..9; we use m = 1..9 only

    // Iterate over each character in the input string.
    for c in s.chars() {
        // Get the digit value as an integer.
        // (c - '0')
        let d = (c as u8 - b'0') as usize;
        // For each modulus from 1 to 9, update the DP.
        for m in 1..10 {
            // nf: new frequency array for modulus m.
            let mut nf = [0; 9];
            // Starting a new substring with the current digit.
            nf[d % m] = 1;
            // Extend all previously recorded substrings.
            for rem in 0..m {
                // Compute new remainder after appending current digit.
                let new_rem = (rem * 10 + d) % m;
                nf[new_rem] += f[m][rem];
            }
            // Update the DP table for modulus m.
            f[m] = nf;
        }
        // Add to answer the count of substrings for which the modulus equal to 
        // the current digit and remainder 0. Notice: if current digit d is 0,
        // then f[0] is not updated by the loop (it remains 0) which is expected.
        ans += f[d][0];
    }
    ans
}

fn main() -> io::Result<()> {
    // Create a buffered input handle.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read the entire input into a string.
    // The original C++ code uses "cin >> s" so it reads one token at a time.
    // Here, we mimic that by reading all input and then picking the first token.
    reader.read_to_string(&mut input)?;

    // Split the input by whitespace and use the first token as the string s.
    let s = input
        .split_whitespace()
        .next()
        .expect("Expected at least one token in input");

    // Compute the result using our function.
    let ans = count_substrings(s);

    // Write the answer to stdout.
    // The output format exactly matches the original C++ behavior.
    println!("{}", ans);

    Ok(())
}