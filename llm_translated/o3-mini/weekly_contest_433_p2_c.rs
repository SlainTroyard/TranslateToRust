use std::io::{self, Read, Write};
use std::process;
use std::sync::OnceLock;

const MOD: i64 = 1000000007;
const MX: usize = 100000;

// Global once-initialized factorial arrays.
// FACT[i] = i! % MOD, and INV_FACT[i] = modular inverse of i! % MOD.
static FACT: OnceLock<Vec<i64>> = OnceLock::new();
static INV_FACT: OnceLock<Vec<i64>> = OnceLock::new();

/// Fast exponentiation modulo MOD.
fn power(mut x: i64, mut n: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    res
}

/// Initialize the global factorial and inverse factorial arrays.
/// This function is called on first use.
fn init_global() {
    FACT.get_or_init(|| {
        let mut fact = vec![0i64; MX];
        fact[0] = 1;
        for i in 1..MX {
            fact[i] = (fact[i - 1] * (i as i64)) % MOD;
        }
        fact
    });
    INV_FACT.get_or_init(|| {
        let fact = FACT.get().expect("FACT should be initialized already");
        let mut inv_fact = vec![0i64; MX];
        // Using Fermat's little theorem for modular inverse.
        inv_fact[MX - 1] = power(fact[MX - 1], MOD - 2);
        for i in (1..MX).rev() {
            inv_fact[i - 1] = (inv_fact[i] * (i as i64)) % MOD;
        }
        inv_fact
    });
}

/// Compute the combination C(n, m) modulo MOD.
/// Returns 0 if m > n.
fn comb(n: usize, m: usize) -> i64 {
    if m > n {
        return 0;
    }
    let fact = FACT.get().expect("FACT not initialized");
    let inv_fact = INV_FACT.get().expect("INV_FACT not initialized");
    (((fact[n] * inv_fact[m]) % MOD) * inv_fact[n - m]) % MOD
}

/// The main function for the problem.
/// It takes a mutable slice of numbers and the integer k,
/// and returns the computed answer.
fn min_max_sums(nums: &mut [i32], k: usize) -> i32 {
    // Ensure factorials and inverse factorials are initialized.
    init_global();

    // Sort the array in non-decreasing order.
    nums.sort();

    let n = nums.len();
    let mut ans = 0i64;
    let mut s = 1i64;
    for i in 0..n {
        // Convert to i64 to avoid overflow and work modulo MOD.
        let low = nums[i] as i64;
        let high = nums[n - 1 - i] as i64;
        ans = (ans + s * (low + high)) % MOD;
        // Update s: s = (s * 2 - comb(i, k - 1) + MOD) % MOD.
        // k - 1 might be greater than i, in which case comb returns 0.
        s = (s * 2 - comb(i, k.saturating_sub(1)) + MOD) % MOD;
    }
    ans as i32
}

/// Reads an integer from the given iterator.
/// If there is an error, prints an error message and exits.
/// `err_msg` is the error message to be printed on failure.
fn read_int<T: std::str::FromStr, I: Iterator<Item = &str>>(iter: &mut I, err_msg: &str) -> T {
    match iter.next() {
        Some(token) => match token.parse::<T>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("{}", err_msg);
                process::exit(1);
            }
        },
        None => {
            eprintln!("{}", err_msg);
            process::exit(1);
        }
    }
}

fn main() {
    // Read the entire input from stdin.
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read input: {}", err);
        process::exit(1);
    }

    // Split input into tokens separated by whitespace.
    let mut tokens = input.split_whitespace();

    // Read n and k from input.
    let n: usize = read_int(&mut tokens, "Error reading input for n and k");
    let k: usize = read_int(&mut tokens, "Error reading input for n and k");

    // Read the array of n integers.
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        let num: i32 = read_int(&mut tokens, &format!("Error reading input for nums[{}]", i));
        nums.push(num);
    }

    // Compute the result.
    let result = min_max_sums(&mut nums, k);

    // Output the result.
    println!("{}", result);
}