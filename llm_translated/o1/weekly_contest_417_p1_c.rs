// Problem: Weekly Contest 417 Problem 1
// Translated from C to Rust. This program reads an integer k from stdin,
// computes the k-th character based on the same logic as the C code,
// and prints the resulting character to stdout.

use std::io::{self, BufRead};

/// Replicates the C function `kthCharacter(int k)`.
/// It uses an infinite loop and an expanding array approach to fill values
/// until it reaches the index (k-1), then returns the computed character.
fn kth_character(k: i32) -> char {
    // Allocate a vector of size (k + 1), initialized to 0
    let mut l = vec![0i32; (k + 1) as usize];
    // 'a' will count up in powers of two (1 << a)
    let mut a = 0u32;

    loop {
        // j is 2^a, as a shifts left
        let j = 1_usize << a;

        // Fill the array's upper half (i + j) with l[i] + 1
        for i in 0..j {
            l[i + j] = l[i] + 1;

            // If we've filled at least up to index (k-1), return the character
            if (i + j) as i32 >= (k - 1) {
                // '97' is 'a' in ASCII; we wrap around with % 26
                return (97 + (l[(k - 1) as usize] % 26)) as u8 as char;
            }
        }

        // Increase a for the next power-of-two range
        a += 1;
    }
}

fn main() -> io::Result<()> {
    // Read a single integer k from stdin (matching scanf("%d", &k) in C)
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer)?;
    let k: i32 = buffer.trim().parse().expect("Failed to parse integer");

    // Call the function and print the resulting character (matching printf("%c\n", ...))
    println!("{}", kth_character(k));
    Ok(())
}