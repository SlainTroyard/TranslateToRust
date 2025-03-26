use std::io;

/// Checks if the strings can be rearranged into each other by splitting into k parts, sorting each part, and comparing.
///
/// # Arguments
/// * `s` - The first string as a byte slice.
/// * `t` - The second string as a byte slice.
/// * `k` - The number of parts to split each string into.
///
/// # Returns
/// `true` if rearrangement is possible, `false` otherwise.
fn is_possible_to_rearrange(s: &[u8], t: &[u8], k: i32) -> bool {
    let k = k as usize;
    if k == 0 {
        return false; // Handle division by zero as per problem constraints
    }
    let len = s.len() / k;
    let total_len = k * len;

    // Ensure both strings have at least `total_len` bytes to avoid out-of-bounds access
    if s.len() < total_len || t.len() < total_len {
        return false;
    }

    let s_part = &s[0..total_len];
    let t_part = &t[0..total_len];

    // Split strings into chunks of 'len' bytes each
    let chunks_s: Vec<&[u8]> = s_part.chunks_exact(len).collect();
    let chunks_t: Vec<&[u8]> = t_part.chunks_exact(len).collect();

    // Sort chunks lexicographically
    let mut sorted_s = chunks_s.clone();
    sorted_s.sort_unstable();

    let mut sorted_t = chunks_t;
    sorted_t.sort_unstable();

    // Compare sorted chunks
    sorted_s == sorted_t
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tokens = input.split_whitespace();

    // Read the first string (s)
    let s = tokens.next().expect("No s string").as_bytes().to_vec();
    // Read the second string (t)
    let t = tokens.next().expect("No t string").as_bytes().to_vec();
    // Read the integer (k)
    let k = tokens.next().expect("No k").parse::<i32>().expect("Invalid k");

    let result = is_possible_to_rearrange(&s, &t, k);
    println!("{}", if result { "true" } else { "false" });
}