use std::io::{self, Write};

fn main() {
    // Input the strings s, t, and the integer k
    let mut s = String::new();
    let mut t = String::new();
    let mut k_str = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read line for s");
    io::stdin().read_line(&mut t).expect("Failed to read line for t");
    io::stdin().read_line(&mut k_str).expect("Failed to read line for k");

    let s = s.trim_end(); // Remove trailing newline
    let t = t.trim_end(); // Remove trailing newline
    let k: usize = k_str.trim().parse().expect("Failed to parse k");

    // Check if it is possible to rearrange the strings
    if is_possible_to_rearrange(s, t, k) {
        println!("true");
    } else {
        println!("false");
    }
}

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let len = s.len() / k;

    // Split the strings into chunks of length `len`
    let s_chunks: Vec<&str> = s.as_bytes().chunks(len).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();
    let t_chunks: Vec<&str> = t.as_bytes().chunks(len).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();

    // Sort the chunks lexicographically
    let mut s_sorted = s_chunks.clone();
    let mut t_sorted = t_chunks.clone();
    s_sorted.sort_unstable();
    t_sorted.sort_unstable();

    // Compare the sorted chunks
    s_sorted == t_sorted
}