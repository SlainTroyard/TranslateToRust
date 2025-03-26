// Problem: Weekly Contest 430 Problem 4
// Translated from C to Rust

use std::io::{self, Write};

/// The modulo constant used for all operations.
const MOD: i64 = 1_000_000_007;

/// Multiplies two numbers under modulo.
fn mult(n: i64, m: i64) -> i64 {
    let nm = (n as i128) * (m as i128);
    (nm % (MOD as i128)) as i64
}

/// Computes n^m under modulo using exponentiation by squaring.
fn power(mut n: i64, mut m: i64) -> i64 {
    n %= MOD;
    let mut res = 1_i64;
    while m > 0 {
        if (m & 1) == 1 {
            res = mult(res, n);
        }
        n = mult(n, n);
        m >>= 1;
    }
    res
}

/// Computes the modular multiplicative inverse of n under modulo.
/// Using Fermat's Little Theorem: inv(n) = n^(MOD-2) mod MOD
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

/// Computes factorial(n) under modulo.
fn factorial(n: i64) -> i64 {
    let mut res = 1_i64;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

/// Computes the binomial coefficient C(n, m) under modulo.
fn comb(n: i64, m: i64) -> i64 {
    // comb(n, m) = factorial(n) / (factorial(m) * factorial(n - m))
    //            = factorial(n) * inv(factorial(m) * factorial(n - m)) mod MOD
    let denom = mult(factorial(m), factorial(n - m));
    mult(factorial(n), inv(denom))
}

/// Computes the count of "good arrays" as defined by the problem statement.
fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    // countGoodArrays(n, m, k) =
    //     comb(n - 1, (n - 1) - k) * m * (m - 1)^((n - 1) - k) mod MOD
    let c = comb(n - 1, (n - 1) - k);
    let part1 = mult(c, m);
    mult(part1, power(m - 1, (n - 1) - k))
}

fn main() {
    // We will read three integers n, m, k from stdin
    // matching the C code's scanf("%d %d %d", &n, &m, &k);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.split_whitespace();

    // Parse n, m, k as i64 (original code uses int for n, m, k,
    // but i64 is fine here because the result is stored in a long long).
    let n: i64 = parts.next().expect("Missing input for n").parse().expect("Invalid input for n");
    let m: i64 = parts.next().expect("Missing input for m").parse().expect("Invalid input for m");
    let k: i64 = parts.next().expect("Missing input for k").parse().expect("Invalid input for k");

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Print the result (mimics printf("%lld\n", result))
    // which is just the result and a newline.
    println!("{}", result);
}