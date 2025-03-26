use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input strings s and t, and the integer k
    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap();
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Check if it is possible to rearrange the strings
    if is_possible_to_rearrange(&s, &t, k) {
        println!("true");
    } else {
        println!("false");
    }
}

/// Function to compare two slices of a string up to a given length.
/// This function is used as a comparator for sorting.
fn cmp(a: &[u8], b: &[u8], len: usize) -> std::cmp::Ordering {
    a[..len].cmp(&b[..len])
}

/// Function to check if it is possible to rearrange the strings s and t
/// into each other by sorting their substrings of length `len`.
fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let len = s.len() / k;

    // Convert strings to byte vectors for sorting
    let mut s_bytes: Vec<u8> = s.as_bytes().to_vec();
    let mut t_bytes: Vec<u8> = t.as_bytes().to_vec();

    // Sort the substrings
    sort_substrings(&mut s_bytes, len, k);
    sort_substrings(&mut t_bytes, len, k);

    // Compare the sorted strings
    s_bytes == t_bytes
}

/// Helper function to sort the substrings of a byte vector.
fn sort_substrings(bytes: &mut [u8], len: usize, k: usize) {
    // Create a vector of substrings
    let mut substrings: Vec<&mut [u8]> = (0..k)
        .map(|i| &mut bytes[i * len..(i + 1) * len])
        .collect();

    // Sort the substrings using the custom comparator
    substrings.sort_unstable_by(|a, b| cmp(a, b, len));

    // Reconstruct the byte vector from the sorted substrings
    for (i, sub) in substrings.iter().enumerate() {
        bytes[i * len..(i + 1) * len].copy_from_slice(sub);
    }
}