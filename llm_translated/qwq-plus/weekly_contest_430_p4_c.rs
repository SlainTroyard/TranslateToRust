const MOD: i64 = 1_000_000_007;

fn mult(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

fn power(base: i64, exponent: i64) -> i64 {
    let mut res = 1;
    let mut current_base = base;
    let mut e = exponent;
    while e > 0 {
        if e & 1 == 1 {
            res = mult(res, current_base);
        }
        current_base = mult(current_base, current_base);
        e >>= 1;
    }
    res
}

fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

fn factorial(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

fn comb(n: i64, m: i64) -> i64 {
    let numerator = factorial(n);
    let denominator = mult(factorial(m), factorial(n - m));
    let inv_denominator = inv(denominator);
    mult(numerator, inv_denominator)
}

fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    let term = (n - 1) - k;
    let c = comb(n - 1, term);
    let term1 = mult(c, m);
    mult(term1, power(m - 1, term))
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = nums[0];
    let m = nums[1];
    let k = nums[2];
    let result = count_good_arrays(n, m, k);
    println!("{}", result);
}