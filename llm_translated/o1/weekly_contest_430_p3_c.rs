use std::{
    collections::HashMap,
    io::{self, BufRead},
};

/// Reads all whitespace-separated tokens from stdin into an iterator of Strings.
/// This mimics the behavior of consecutive scanf calls in C (ignoring all whitespace).
fn read_all_tokens() -> std::vec::IntoIter<String> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();
    let tokens: Vec<String> = lines
        .iter()
        .flat_map(|line| line.split_whitespace().map(|s| s.to_string()))
        .collect();
    tokens.into_iter()
}

/// Compute the greatest common divisor of two integers.
/// This function emulates the C gcd function exactly.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Translated version of the C function `numberOfSubsequences(int* nums, int numsSize)`.
/// Takes a slice of integers and returns a 64-bit integer result.
fn number_of_subsequences(nums: &[i32]) -> i64 {
    let nums_size = nums.len();
    const HASH_SIZE: usize = 100003; // Not strictly used in this Rust version, but kept for clarity

    // We use a HashMap instead of a custom linked-list-based hash table.
    // Keys (i32) map to cumulative counts (i64).
    let mut hash_map: HashMap<i32, i64> = HashMap::with_capacity(HASH_SIZE);

    // Helper functions to emulate the same behavior as the C code's hash_* functions.
    let mut hash_insert = |key: i32, value: i64| {
        *hash_map.entry(key).or_insert(0) += value;
    };

    let hash_get = |key: i32| -> i64 {
        *hash_map.get(&key).unwrap_or(&0)
    };

    // Emulate hash_clear
    let mut hash_clear = || {
        hash_map.clear();
    };

    // Clear the map at the beginning
    hash_clear();

    let mut ans: i64 = 0;

    // First set of loops: populate the hash table
    // for (i = 4; i < numsSize - 2; i++) { ... }
    for i in 4..(nums_size.saturating_sub(2)) {
        let c = nums[i];
        for j in (i + 2)..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            // key = ((d/g) << 16) | (c/g)
            let key = ((d / g) << 16) | (c / g);
            hash_insert(key, 1);
        }
    }

    // Second set of loops: query and update the hash table
    // for (i = 2; i < numsSize - 4; i++) { ... }
    for i in 2..(nums_size.saturating_sub(4)) {
        let b = nums[i];

        // for (j = 0; j < i - 1; j++) { ... ans += hash_get(key); }
        for j in 0..(i.saturating_sub(1)) {
            let a = nums[j];
            let g = gcd(a, b);
            let key = ((a / g) << 16) | (b / g);
            ans += hash_get(key);
        }

        // for (j = i + 4; j < numsSize; j++) { ... hash_insert(key, -1); }
        let c = nums[i + 2];
        for j in (i + 4)..nums_size {
            let d = nums[j];
            let g = gcd(c, d);
            let key = ((d / g) << 16) | (c / g);
            hash_insert(key, -1);
        }
    }

    // Final hash_clear
    hash_clear();

    ans
}

fn main() {
    // Read all tokens from stdin
    let mut tokens = read_all_tokens();

    // 1) Read n
    let n: usize = tokens
        .next()
        .expect("Expected an integer for n")
        .parse()
        .expect("Failed to parse n as an integer");

    // 2) Read n integers into a vector
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let val: i32 = tokens
            .next()
            .expect("Expected more integers in input")
            .parse()
            .expect("Failed to parse an integer in the array");
        nums.push(val);
    }

    // Call the translated function
    let result = number_of_subsequences(&nums);

    // Print the result (matching "printf("%lld\n", result);" in C)
    println!("{}", result);
}