use std::collections::HashMap;
use std::io::{self, BufRead};

fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    let n = s.len();
    let size = n / k;
    let mut mp = HashMap::new();

    // Count the substrings of `s` and `t` in the map
    for i in (0..n).step_by(size) {
        *mp.entry(&s[i..i + size]).or_insert(0) += 1;
        *mp.entry(&t[i..i + size]).or_insert(0) -= 1;
    }

    // Check if all counts are zero
    mp.values().all(|&v| v == 0)
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input
    let s = lines.next().unwrap().expect("Failed to read s");
    let t = lines.next().unwrap().expect("Failed to read t");
    let k: usize = lines.next().unwrap().expect("Failed to read k").parse().expect("Failed to parse k");

    // Call the function and get the result
    let result = is_possible_to_rearrange(&s, &t, k);

    // Output the result to stdout
    if result {
        println!("true");
    } else {
        println!("false");
    }
}