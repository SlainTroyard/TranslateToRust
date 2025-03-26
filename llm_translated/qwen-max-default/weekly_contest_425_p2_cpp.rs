use std::collections::HashMap;
use std::io::{self, BufRead};

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let n = s.len();
    let size = n / k;
    let mut mp = HashMap::new();

    // Count substrings of length `size` in `s`
    for i in (0..n).step_by(size) {
        *mp.entry(&s[i..i + size]).or_insert(0) += 1;
    }

    // Subtract counts for substrings of length `size` in `t`
    for i in (0..n).step_by(size) {
        *mp.entry(&t[i..i + size]).or_insert(0) -= 1;
    }

    // Check if all counts are zero
    mp.values().all(|&v| v == 0)
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap();
    let k: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Call the function and get the result
    let result = is_possible_to_rearrange(&s, &t, k);

    // Print the result to stdout
    if result {
        println!("true");
    } else {
        println!("false");
    }
}