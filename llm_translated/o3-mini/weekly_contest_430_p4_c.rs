use std::io::{self, Read};

// Define the modulo constant
const MOD: i64 = 1_000_000_007;

// Multiply two numbers modulo MOD
fn mult(n: i64, m: i64) -> i64 {
    (n * m) % MOD
}

// Efficient modular exponentiation: computes n^m % MOD
fn power(n: i64, m: i64) -> i64 {
    let mut res: i64 = 1;
    let mut base = n % MOD;
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

// Computes modular inverse of n modulo MOD using Fermat's little theorem,
// since MOD is prime. This returns n^(MOD-2) mod MOD.
fn inv(n: i64) -> i64 {
    power(n, MOD - 2)
}

// Computes factorial of n modulo MOD
fn factorial(n: i64) -> i64 {
    let mut res = 1;
    for i in 2..=n {
        res = mult(res, i);
    }
    res
}

// Computes combination C(n, m) modulo MOD
fn comb(n: i64, m: i64) -> i64 {
    let numerator = factorial(n);
    let denominator = mult(factorial(m), factorial(n - m));
    mult(numerator, inv(denominator))
}

// Computes countGoodArrays as per the given logic
fn count_good_arrays(n: i64, m: i64, k: i64) -> i64 {
    let choose = comb(n - 1, n - 1 - k);
    let power_part = power(m - 1, n - 1 - k);
    mult(mult(choose, m), power_part)
}

fn main() -> io::Result<()> {
    // Read entire input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    // Parse input: The input may consist of multiple lines or multiple values per line.
    // We'll split the string by whitespace and parse exactly three integers.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 3 {
        eprintln!("Not enough input values provided. Expected at least 3 integers.");
        std::process::exit(1);
    }

    // Parse n, m, k; handle errors gracefully
    let n: i64 = tokens[0].parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse 'n' as an integer.");
        std::process::exit(1);
    });

    let m: i64 = tokens[1].parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse 'm' as an integer.");
        std::process::exit(1);
    });

    let k: i64 = tokens[2].parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse 'k' as an integer.");
        std::process::exit(1);
    });

    // Calculate the result using the provided algorithm
    let result = count_good_arrays(n, m, k);

    // Output the result in the exact same format as the original code (a single integer followed by a newline)
    println!("{}", result);

    Ok(())
}