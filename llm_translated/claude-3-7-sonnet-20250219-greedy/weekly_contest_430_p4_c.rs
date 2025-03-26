// Problem: Weekly Contest 430 Problem 4
use std::io;

const MOD: i64 = 1_000_000_007;

fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

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

fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    let n = n as i64;
    let m = m as i64;
    let k = k as i64;
    
    mult(mult(comb(n - 1, n - 1 - k), m), power(m - 1, n - 1 - k))
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = values[0];
    let m = values[1];
    let k = values[2];
    
    // Calculate the result
    let result = count_good_arrays(n, m, k);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}