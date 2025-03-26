// Problem: Weekly Contest 430 Problem 4
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

// Multiply two numbers and take modulo
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

// Calculate power using binary exponentiation
fn power(n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut base = n;
    let mut m = m;
    while m > 0 {
        if m & 1 == 1 {
            res = mult(res, base);
        }
        base = mult(base, base);
        m >>= 1;
    }
    res
}

// Calculate modular inverse
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

// Calculate factorial
fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

// Calculate combination
fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

// Calculate the number of good arrays
fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    mult(
        mult(
            comb(n - 1, n - 1 - k),
            m as i64
        ),
        power(m as i64 - 1, n as i64 - 1 - k)
    )
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse input
    let mut iter = input.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    // Calculate the result
    let result = count_good_arrays(n, m, k);

    // Output the result
    println!("{}", result);

    Ok(())
}