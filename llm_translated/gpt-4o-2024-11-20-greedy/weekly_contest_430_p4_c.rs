use std::io::{self, Write};

const MOD: i64 = 1_000_000_007;

// Function to perform modular multiplication
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

// Function to perform modular exponentiation
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

// Function to calculate modular multiplicative inverse
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

// Function to calculate factorial modulo MOD
fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

// Function to calculate combinations (n choose m) modulo MOD
fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

// Function to calculate the result for the problem
fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    mult(
        mult(comb(n - 1, n - 1 - k), m),
        power(m - 1, n - 1 - k),
    )
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.trim().split_whitespace();

    // Parse input values
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let m: i64 = iter.next().unwrap().parse().expect("Invalid input for m");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Output the result
    println!("{}", result);
}