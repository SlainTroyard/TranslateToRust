```rust
use std::collections::HashSet;
use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

// Function to calculate modular exponentiation
fn mod_pow(base: i64, exp: usize, modulus: i64) -> Vec<i64> {
    let mut pow_base = vec![1; exp + 1];
    for i in 0..exp {
        pow_base[i + 1] = (pow_base[i] * base % modulus) as i64;
    }
    pow_base
}

fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let n = target.len();
    let target: Vec<char> = target.chars().collect();

    const MOD: i64 = 1_070_777_777;

    // Generate a random BASE value
    let mut rng = rand::thread_rng();
    let base: i64 = (8e8 as i64 + (rand::random::<u64>() % (1e*80`

(N