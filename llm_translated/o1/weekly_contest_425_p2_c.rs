/// A direct translation of the C code from LeetCode Weekly Contest 425 Problem 2 into Rust.
/// This program reads two strings (s, t) and an integer k from stdin, then checks if it's
/// possible to rearrange s and t such that they match when each is divided into k chunks
/// of equal length and those chunks are sorted lexicographically.

use std::error::Error;
use std::io::{self, Read};

/// Checks if it is possible to rearrange the strings s and t into the same configuration
/// by splitting each into k chunks of equal length and sorting those chunks.
fn isPossibleToRearrange(s: &str, t: &str, k: usize) -> bool {
    // If lengths differ, it's immediately impossible to rearrange them to match.
    if s.len() != t.len() {
        return false;
    }

    // If k is zero or s.len() is not divisible by k, this won't work safely.
    if k == 0 || s.len() % k != 0 {
        return false;
    }

    // Each chunk will have length = total_length / k
    let len = s.len() / k;

    // Collect k chunks from s and t
    let mut chunks_s: Vec<&str> = (0..k).map(|i| &s[i * len..(i + 1) * len]).collect();
    let mut chunks_t: Vec<&str> = (0..k).map(|i| &t[i * len..(i + 1) * len]).collect();

    // Sort the chunks lexicographically
    chunks_s.sort_unstable();
    chunks_t.sort_unstable();

    // If all sorted chunks match, the strings can be rearranged the same way
    chunks_s == chunks_t
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read all input from stdin as a single string
    let mut input_string = String::new();
    io::stdin().read_to_string(&mut input_string)?;

    // Tokenize the input by whitespace.
    // The C code uses scanf("%s", s), scanf("%s", t), scanf("%d", &k)
    // which implies reading s, t, and k from three separate whitespace-delimited tokens.
    let mut tokens = input_string.split_whitespace();

    // Extract s, t, and k from the tokens. Return an error if something is missing or invalid.
    let s = tokens.next().ok_or("Expected input for s")?;
    let t = tokens.next().ok_or("Expected input for t")?;
    let k = tokens
        .next()
        .ok_or("Expected input for k")?
        .parse::<usize>()
        .map_err(|_| "Invalid integer for k")?;

    // Check if it is possible to rearrange
    if isPossibleToRearrange(s, t, k) {
        println!("true");
    } else {
        println!("false");
    }

    Ok(())
}