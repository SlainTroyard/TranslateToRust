// Problem: Weekly Contest 430 Problem 4
use std::io;

const MOD: i64 = 1_000_000_007;

/// Multiply two numbers under modulo
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

/// Calculate power with modulo
fn power(mut n: i64, mut m: i64) -> i64 {
    let mut res = 1;
    let mut base = n;
    while m > 0 {
        if m & 1 == 1 {
            res = mult(res, base);
        }
        base = mult(base, base);
        m >>= 1;
    }
    res
}

/// Calculate the modular inverse
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

/// Calculate factorial under modulo
fn factorial(n: i64) -> i64 {
    (2..=n).fold(1, |res, i| mult(res, i))
}

/// Calculate combination (n choose m) under modulo
fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

/// Count the number of good arrays
fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    mult(mult(comb(n - 1, n - 1 - k), m), power(m - 1, n - 1 - k))
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: i64 = iter.next().unwrap().parse().expect("Failed to parse m");
    let k: i64 = iter.next().unwrap().parse().expect("Failed to parse k");

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Output the result
    println!("{}", result);
}