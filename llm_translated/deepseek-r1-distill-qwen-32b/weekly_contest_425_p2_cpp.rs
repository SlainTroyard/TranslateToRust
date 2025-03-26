use std::collections::HashMap;
use std::io;

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let n = s.len();
    if k == 0 || n % k != 0 {
        return false;
    }

    let size = n / k;
    let mut chunk_counts = HashMap::new();

    // Count chunks from string s
    for i in (0..n).step_by(size) {
        let chunk = &s[i..i + size];
        *chunk_counts.entry(chunk.to_string()).or_insert(0) += 1;
    }

    // Subtract chunks from string t
    for i in (0..n).step_by(size) {
        let chunk = &t[i..i + size];
        *chunk_counts.entry(chunk.to_string()).or_insert(0) -= 1;
    }

    // Check if all chunk counts are zero
    for count in chunk_counts.values() {
        if *count != 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read s");
    s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read t");
    t = t.trim().to_string();

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read k");
    let k = k_str.trim().parse::<usize>().expect("Invalid k");

    let result = is_possible_to_rearrange(&s, &t, k);

    if result {
        println!("true");
    } else {
        println!("false");
    }
}