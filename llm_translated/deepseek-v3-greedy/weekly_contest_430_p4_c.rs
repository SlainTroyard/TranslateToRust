use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;

// Multiplies two numbers under modulo MOD
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

// Computes n^m under modulo MOD using exponentiation by squaring
fn power(n: i64, mut m: i64) -> i64 {
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

// Computes the modular inverse of n under modulo MOD
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

// Computes the factorial of n under modulo MOD
fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

// Computes the combination C(n, m) under modulo MOD
fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

// Counts the number of good arrays based on the given parameters
fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    mult(mult(comb(n - 1, n - 1 - k), m), power(m - 1, n - 1 - k))
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    // Parse the input into integers
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input");
    let m: i64 = iter.next().unwrap().parse().expect("Invalid input");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input");

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Output the result
    println!("{}", result);
}