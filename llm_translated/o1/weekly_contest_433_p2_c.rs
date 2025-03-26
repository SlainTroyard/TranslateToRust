// LeetCode Weekly Contest 433 Problem 2 in Rust
//
// REQUIREMENTS (as per the prompt):
// 1. Translates the ENTIRE C code as a complete program
// 2. Preserves the same algorithm logic exactly
// 3. Uses idiomatic Rust with proper error handling
// 4. Maintains the EXACT SAME stdin/stdout format
// 5. Adds helpful comments where needed

use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_000;

// Fast exponentiation: computes (x^n) % m
fn power(mut x: i64, mut n: i64, m: i64) -> i64 {
    let mut res = 1i64;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

// Combination function comb(n, m) = n! / (m! * (n-m)!) mod MOD
// Returns 0 if m > n or m < 0
fn comb(n: i32, m: i32, f: &[i64; MX], inv_f: &[i64; MX]) -> i64 {
    if m > n || m < 0 {
        return 0;
    }
    let nu = n as usize;
    let mu = m as usize;
    ((f[nu] * inv_f[mu]) % MOD) * inv_f[nu - mu] % MOD
}

// We use an on-demand initialization for factorial arrays
// and their modular inverses, similar to static initialization in C.
fn get_factorial_arrays() -> (&'static [i64; MX], &'static [i64; MX]) {
    use std::sync::OnceLock;

    static F_ONCE: OnceLock<[i64; MX]> = OnceLock::new();
    static INV_F_ONCE: OnceLock<[i64; MX]> = OnceLock::new();

    let f = F_ONCE.get_or_init(|| {
        let mut arr = [0i64; MX];
        arr[0] = 1;
        for i in 1..MX {
            arr[i] = (arr[i - 1] * i as i64) % MOD;
        }
        arr
    });

    let inv_f = INV_F_ONCE.get_or_init(|| {
        let mut arr = [0i64; MX];
        let f_ref = f; // reference to already initialized factorial array
        arr[MX - 1] = power(f_ref[MX - 1], MOD - 2, MOD); // Fermat's little theorem
        for i in (1..MX).rev() {
            arr[i - 1] = (arr[i] * i as i64) % MOD;
        }
        arr
    });

    (f, inv_f)
}

// Translated from: int minMaxSums(int* nums, int numsSize, int k)
fn min_max_sums(nums: &mut [i32], k: i32) -> i32 {
    // Ensure factorial arrays are initialized
    let (f, inv_f) = get_factorial_arrays();

    // Sort the array (qsort in C)
    nums.sort();

    let n = nums.len();
    let mut ans = 0i64;
    let mut s = 1i64;

    // Reproduce the logic from the C code
    for i in 0..n {
        // (nums[i] + nums[n-1-i]) is the sum of the i-th smallest and i-th largest
        let mm_sum = (nums[i] as i64 + nums[n - 1 - i] as i64) % MOD;
        ans = (ans + s * mm_sum) % MOD;

        // Update multiplier s:
        // s = (s * 2 - comb(i, k - 1) + MOD) % MOD
        // Here i is a usize, but comb wants i32, so convert carefully
        let ci = i as i32;
        let tmp = comb(ci, k - 1, f, inv_f);
        s = (s * 2 - tmp + MOD) % MOD;
    }

    // Return ans as int
    ans as i32
}

// Helper function to parse the next integer from an iterator, printing the
// appropriate error message if anything fails.
fn parse_next_int(iter: &mut impl Iterator<Item = &str>, index_for_error: Option<usize>) -> i32 {
    match iter.next() {
        Some(token) => match token.parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                if let Some(i) = index_for_error {
                    eprintln!("Error reading input for nums[{}]", i);
                } else {
                    eprintln!("Error reading input for n and k");
                }
                std::process::exit(1);
            }
        },
        None => {
            if let Some(i) = index_for_error {
                eprintln!("Error reading input for nums[{}]", i);
            } else {
                eprintln!("Error reading input for n and k");
            }
            std::process::exit(1);
        }
    }
}

fn main() {
    // Read entire stdin into a string (since C code with scanf can handle values over multiple lines)
    let mut input_string = String::new();
    if io::stdin().read_to_string(&mut input_string).is_err() {
        eprintln!("Error reading input for n and k");
        std::process::exit(1);
    }

    // Split input by whitespace
    let mut tokens = input_string.split_whitespace();

    // Parse n and k
    let n = parse_next_int(&mut tokens, None);
    let k = parse_next_int(&mut tokens, None);

    // Allocate a vector and read n integers
    let mut nums = Vec::new();
    nums.reserve(n as usize);
    for i in 0..n {
        let val = parse_next_int(&mut tokens, Some(i as usize));
        nums.push(val);
    }

    // Call the translated function
    let result = min_max_sums(&mut nums, k);

    // Print the result
    println!("{}", result);
}