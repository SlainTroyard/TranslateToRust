use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;

// Function to multiply two numbers under modulo MOD
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

// Function to compute n^m under modulo MOD using exponentiation by squaring
fn power(n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut base = n;
    let mut exponent = m;
    while exponent > 0 {
        if exponent & 1 == 1 {
            res = mult(res, base);
        }
        base = mult(base, base);
        exponent >>= 1;
    }
    res
}

// Function to compute the modular inverse of n under modulo MOD
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

// Function to compute the factorial of n under modulo MOD
fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

// Function to compute the combination C(n, m) under modulo MOD
fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

// Function to count the number of good arrays
fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    mult(mult(comb(n - 1, n - 1 - k), m), power(m - 1, n - 1 - k))
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input");
    let m: i64 = iter.next().unwrap().parse().expect("Invalid input");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input");

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Output the result
    println!("{}", result);
}