use std::collections::HashMap;

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let n = s.len();
    // Handle division by zero if k is 0 (as original code may crash)
    let size = if k == 0 { 0 } else { n / k };

    let mut chunk_counts = HashMap::new();

    // Process each chunk in s and count occurrences
    for i in (0..n).step_by(size) {
        let end = (i + size).min(s.len());
        let chunk = &s[i..end];
        *chunk_counts.entry(chunk.to_string()).or_insert(0) += 1;
    }

    // Process each chunk in t and subtract counts
    for i in (0..n).step_by(size) {
        let end = (i + size).min(t.len());
        let chunk = &t[i..end];
        *chunk_counts.entry(chunk.to_string()).or_insert(0) -= 1;
    }

    // Check if all counts are zero
    chunk_counts.values().all(|&count| count == 0)
}

fn main() {
    // Read entire input and split into whitespace-separated tokens
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse inputs with default values (matching original behavior)
    let s = tokens.next().unwrap_or_default().to_string();
    let t = tokens.next().unwrap_or_default().to_string();
    let k: usize = tokens.next()
        .and_then(|s| s.parse().ok())
        .unwrap_or_default();

    let result = is_possible_to_rearrange(&s, &t, k);

    // Print result in exact original format
    println!("{}", if result { "true" } else { "false" });
}