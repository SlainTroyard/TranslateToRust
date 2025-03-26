use std::collections::HashMap;
use std::io;

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let n = s.len();
    let size = n / k;
    let mut mp: HashMap<&str, i32> = HashMap::new();

    // Count occurrences of each substring in s
    for i in (0..n).step_by(size) {
        let substring = &s[i..i + size];
        *mp.entry(substring).or_insert(0) += 1;
    }

    // Subtract occurrences of each substring in t
    for i in (0..n).step_by(size) {
        let substring = &t[i..i + size];
        *mp.entry(substring).or_insert(0) -= 1;
    }

    // Check if all counts are zero
    mp.values().all(|&value| value == 0)
}

fn main() {
    let mut input = String::new();

    // Read the first string
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();

    // Read the second string
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = input.trim();

    // Read the integer k
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Failed to parse k");

    // Check if rearrangement is possible
    let result = is_possible_to_rearrange(s, t, k);

    // Print the result
    if result {
        println!("true");
    } else {
        println!("false");
    }
}