use std::error::Error;
use std::io::{self, BufRead, Write};

/// Computes the length after transformations, following the same logic as the C code.
/// The transformation involves shifting counts in a 26-element array.
fn length_after_transformations(s: &str, t: usize) -> i32 {
    // The modulo constant as defined in the problem.
    const MOD: i64 = 1_000_000_007;

    // Use a fixed-size array for counts. Using i64 here to avoid potential overflows.
    let mut lst = [0i64; 26];

    // Count the frequency of each lowercase letter from the input string.
    // This mimics the scanf("%s", s) in C, reading token by token.
    for ch in s.chars() {
        // Only process valid lowercase letters (assuming input follows the problem constraints).
        let idx = (ch as u8).wrapping_sub(b'a') as usize;
        if idx < 26 {
            lst[idx] += 1;
        }
    }

    // Perform t transformations as described in the problem.
    for _ in 0..t {
        // Store the last element, which will be used in the shifting.
        let z = lst[25];
        // Shift the counts for indices 25 down to 2.
        for i in (2..=25).rev() {
            lst[i] = lst[i - 1];
        }
        // For index 1, combine lst[0] and the saved value z, taking modulo.
        lst[1] = (lst[0] + z) % MOD;
        // For index 0, set it to the saved value.
        lst[0] = z;
    }

    // Sum up all counts modulo MOD to obtain the final answer.
    let ans = lst.iter().fold(0, |acc, &val| (acc + val) % MOD);
    ans as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    // Prepare to read lines from standard input.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first token which is the string s.
    // This follows the scanf("%s", s) behavior from C.
    let s: String = {
        let line = lines
            .next()
            .ok_or("Missing input for string")??;
        // Take the first token in the line to mimic the behavior of %s in C.
        line.split_whitespace()
            .next()
            .ok_or("Missing string token")?
            .to_string()
    };

    // Read the integer t from the next token.
    let t: usize = {
        // Read the next line.
        let line = lines
            .next()
            .ok_or("Missing input for integer")??;
        // Take the first token in the line.
        line.split_whitespace()
            .next()
            .ok_or("Missing integer token")?
            .parse()?
    };

    // Compute the result using the transformation function.
    let result = length_after_transformations(&s, t);

    // Print the result to stdout. This matches the printf("%d", result) of C.
    println!("{}", result);

    Ok(())
}