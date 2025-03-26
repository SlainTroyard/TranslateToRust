use std::io::{self, BufRead};

const MOD: u64 = 1_000_000_007;

// Multiplies two numbers modulo MOD
fn mult(n: u64, m: u64) -> u64 {
    (n * m) % MOD
}

// Exponentiation by squaring modulo MOD
fn power(n: u64, m: u64) -> u64 {
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

// Modular multiplicative inverse using Fermat's little theorem
fn inv(n: u64) -> u64 {
    power(n, MOD - 2)
}

// Computes factorial modulo MOD
fn factorial(n: u64) -> u64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }

    res
}

// Computes n choose m modulo MOD
fn comb(n: u64, m: u64) -> u64 {
    if m > n {
        return 0; // Invalid case
    }

    mult(
        factorial(n),
        inv(mult(factorial(m), factorial(n - m))),
    )
}

// Counts the good arrays based on the given formula
fn count_good_arrays(n: u64, m: u64, k: u64) -> u64 {
    mult(
        mult(comb(n - 1, n - 1 - k), m),
        power(m - 1, n - 1 - k),
    )
}

fn main() -> io::Result<()> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the input values
    if let Some(Ok(line)) = lines.next() {
        let parts: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        if parts.len() == 3 {
            let n = parts[0];
            let m = parts[1];
            let k = parts[2];

            // Calculate the result
            let result = count_good_arrays(n, m, k);

            // Print the result
            println!("{}", result);
        }
    }

    Ok(())
}