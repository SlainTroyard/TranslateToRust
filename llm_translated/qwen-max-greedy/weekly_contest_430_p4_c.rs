// Problem: Weekly Contest 430 Problem 4
use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

/// Multiplies two numbers and returns the result modulo `MOD`.
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

/// Computes n^m % MOD using binary exponentiation.
fn power(n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut base = n;
    let mut exp = m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = mult(res, base);
        }
        base = mult(base, base);
        exp >>= 1;
    }
    res
}

/// Computes the modular inverse of n using Fermat's Little Theorem.
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

/// Computes n! % MOD.
fn factorial(n: i64) -> i64 {
    (2..=n).fold(1, |acc, x| mult(acc, x))
}

/// Computes the binomial coefficient (n choose m) % MOD.
fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

/// Computes the number of good arrays.
fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    mult(mult(comb(n - 1, n - 1 - k), m), power(m - 1, n - 1 - k))
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect();

    // Ensure we have exactly three values
    if values.len() != 3 {
        eprintln!("Expected exactly three integers as input");
        return;
    }

    let n = values[0];
    let m = values[1];
    let k = values[2];

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Output the result
    println!("{}", result);
}