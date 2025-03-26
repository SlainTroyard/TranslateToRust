use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

/// Multiply two numbers with modulo
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

/// Calculate (n^m) % MOD efficiently using binary exponentiation
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

/// Calculate the modular multiplicative inverse using Fermat's Little Theorem
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

/// Calculate factorial(n) % MOD
fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

/// Calculate nCm (binomial coefficient) % MOD
fn comb(n: i64, m: i64) -> i64 {
    mult(factorial(n), inv(mult(factorial(m), factorial(n - m))))
}

/// Count good arrays with the given parameters
fn count_good_arrays(n: i32, m: i32, k: i32) -> i64 {
    let n = n as i64;
    let m = m as i64;
    let k = k as i64;
    
    mult(mult(comb(n - 1, n - 1 - k), m), power(m - 1, n - 1 - k))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap()?;
    
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Expected 3 integers".into());
    }

    let n: i32 = parts[0].parse()?;
    let m: i32 = parts[1].parse()?;
    let k: i32 = parts[2].parse()?;

    // Calculate the result
    let result = count_good_arrays(n, m, k);
    
    // Output the result
    println!("{}", result);
    
    Ok(())
}