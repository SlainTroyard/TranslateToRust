use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, BufRead};

// Define the modulus as per the C++ code
const MOD: u64 = 1_070_777_777;

// Struct to hold a dynamic HashSet for prefix hashes
#[derive(Default)]
struct HashSetWrapper {
    data: Vec<u32>,
}

impl HashSetWrapper {
    // Add a hash value to the HashSet
    fn add_hash(&mut self, hash: u32) {
        self.data.push(hash);
    }

    // Sort the set and remove duplicates
    fn sort_and_unique(&mut self) {
        self.data.sort_unstable();
        self.data.dedup();
    }
}

// Generate a random base between 800,000,000 and 900,000,000
fn get_random_base() -> u64 {
    const BASE_MIN: u64 = 800_000_000;
    const BASE_RANGE: u64 = 100_000_000;
    let mut rng = rand::thread_rng();
    let rand_num: u64 = rand::random::<u64>();
    BASE_MIN + rand_num % BASE_RANGE
}

fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let n = target.len();
    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);

    let base = get_random_base();