use std::io;

const MOD: u64 = 1_000_000_007;

fn length_after_transformations(s: &str, t: usize) -> u64 {
    let mut counts = [0u64; 26];
    // Initialize character counts
    for c in s.chars() {
        if c.is_ascii_lowercase() {
            let idx = (c as u8 - b'a') as usize;
            counts[idx] += 1;
        }
    }

    for _ in 0..t {
        let z = counts[25];
        // Shift elements from index 2 to 25 (inclusive) to the right by one
        for i in (2..=25).rev() {
            counts[i] = counts[i - 1];
        }
        // Update the first two elements
        counts[1] = (counts[0] + z) % MOD;
        counts[0] = z;
    }

    // Compute the sum modulo MOD
    counts.iter().sum::<u64>() % MOD
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0];
    let t: usize = parts[1].parse().unwrap();
    println!("{}", length_after_transformations(s, t));
}