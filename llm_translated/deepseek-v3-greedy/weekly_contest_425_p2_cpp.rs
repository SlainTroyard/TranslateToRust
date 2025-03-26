use std::collections::HashMap;
use std::io;

fn main() {
    // Read input strings s and t, and integer k from stdin
    let mut s = String::new();
    let mut t = String::new();
    let mut k_input = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read s");
    io::stdin().read_line(&mut t).expect("Failed to read t");
    io::stdin().read_line(&mut k_input).expect("Failed to read k");

    // Trim whitespace and parse k as an integer
    let s = s.trim();
    let t = t.trim();
    let k: usize = k_input.trim().parse().expect("k must be an integer");

    // Check if rearrangement is possible and print the result
    let result = is_possible_to_rearrange(s, t, k);

    if result {
        println!("true");
    } else {
        println!("false");
    }
}

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let n = s.len();
    let size = n / k;
    let mut mp: HashMap<&str, i32> = HashMap::new();

    // Count occurrences of each substring in s
    for i in (0..n).step_by(size) {
        let substring = &s[i..i + size];
        *mp.entry(substring).or_insert(0) += 1;
    }

    // Decrement counts for each substring in t
    for i in (0..n).step_by(size) {
        let substring = &t[i..i + size];
        *mp.entry(substring).or_insert(0) -= 1;
    }

    // Check if all counts are zero
    mp.values().all(|&value| value == 0)
}